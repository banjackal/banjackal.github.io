---
layout: post
title: "Learning Rust for Use in AWS - Part 2"
categories: AWS, lambda, Rust, serverless
---

## Overview
I spend the past week an a half reading through the [Rust Book](https://doc.rust-lang.org/book/) and I'm going to try a little experiment, comparing the functionality of a rust lambda to a Python lambda.

## The Experiment
I'm going to capture a pretty straightforward lambda function use case with S3, where a user will upload an image to a bucket, and a lambda function will run when the image is uploaded, make a smaller, thumbnail version of the image, and save it to a `thumbnails/' folder in another bucket.

After enabling this functionality, I'm also going to maintain an index in DynamoDB, wherewe can store metadata about the image, including its thumbnail location.

Using 2 different languages for our Lambda functions, we can compare overall resource usage, which could extrapolate to potential cost savings if the Lambda executions span past the [AWS Free Tier](https://aws.amazon.com/free/?all-free-tier.sort-by=item.additionalFields.SortRank&all-free-tier.sort-order=asc&awsf.Free%20Tier%20Types=*all&awsf.Free%20Tier%20Categories=*all&all-free-tier.q=lambda&all-free-tier.q_operator=AND) allocation limits.

## Architecture
In order to build this workflow, we'll need to set up a few things:
- An S3 bucket to store our images
- An S3 event trigger to notify our lambda that a new image has been uploaded
- A DynamoDB to store our metadata
- 2 Lambda functions (one in Python and one in Rust) to do the size conversion and store the relevant metadata

## Infrastructure
Let's get started on configuring our AWS resources. We're going to manage all of our resources in Terraform, which will make our infrastructure easy to create, update, and destroy.
Installation and instructions about how or why to use Terraform is outside the scope of this post. For more information, please refer to the [Terraform website](https://www.terraform.io/).

Let's start by creating an new project, and creating a folder for our Terraform code:
```shell
$ mkdir -p lambda_comparison/terraform
$ cd lambda_comparison/terraform
```

Now, we need to define our resources in our `main.tf`
```shell
touch main.tf
```

Using your editor of choice, add the S3 bucket
```hcl

```

<!-- TODO - document the tf code after it works -->


## Rust code
Install [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html)
Using [Cargo Lambda Runtime](https://github.com/awslabs/aws-lambda-rust-runtime)

```rust
cargo lambda new http-demo
```

```rust
cargo lambda new rust-image-processor
```

```rust
cargo lambda build --release --arm64
```

