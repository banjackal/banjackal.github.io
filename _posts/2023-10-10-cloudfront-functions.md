---
layout: post
title: CloudFront Functions
description: Maintaining S3 Website security by using CloudFront Functions
categories: AWS, CloudFront, Terraform, Javascript
---

In order to maintain a secure, private S3 bucket to host static website content, we need to only use S3 as our file repository and not enable the static website hosting features on the bucket, which would expose the bucket to the public internet. Instead, we use CloudFront to expose, cache, and serve the website data.

With that in mind, certain issues arise which S3 website hosting could address, at the cost of publicly exposing the bucket. In this case, we want to redirect website paths of `https://<host>/<path>` to `https://<host>/<path>/index.html` but also not rewrite the path in the browser bar. This will enable our static site internal links to work properly.

We're going to achieve this using CloudFront functions and manage all of our infrastructure with Terraform.


# CloudFront Functions

CloudFront Functions are lightweight javascript functions you can use to customize your CloudFront behavior, and based on AWS's [documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/edge-functions.html), are perfect for our use case.

# Writing the function

For our use case, we're going to create a simple `page-router.js` file:

```js
function handler(event) {
    var request = event.request;
    var uri = request.uri;

    // Check whether the URI is missing a file name.
    if (uri.endsWith('/')) {
        request.uri += 'index.html';
    }
    // Check whether the URI is missing a file extension.
    else if (!uri.includes('.')) {
        request.uri += '/index.html';
    }

    return request;
}
```

# Adding to Terraform
In our Terraform configuration, we're using the [official Terraform CloudFront Module](https://registry.terraform.io/modules/terraform-aws-modules/cloudfront/aws/latest)

In order to add the function, we only need to add the function resource and create an association on the viewer-request on the ordered_cache_behavior

```
resource "aws_cloudfront_function" "page_router" {
  name    = "<unique name>"
  runtime = "cloudfront-js-1.0"
  code    = file("page-router.js")
}

module "website" {
  source = "terraform-aws-modules/cloudfront/aws"

  ...

  ordered_cache_behavior = [
    {
      ...

      function_association = {
        viewer-request = {
          function_arn = aws_cloudfront_function.page_router.arn
        }
      }
    }
  ]
```

# Testing it out
We've redeployed our Terraform configuration, and now when we navigate to `https://<host>/<page>` we can view the contents of `https://<host>/<page>.index.html`
