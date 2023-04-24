resource "aws_s3_bucket" "images" {
  bucket = "python-rust-image-processing-demo"
}

resource "aws_s3_bucket_public_access_block" "block_public_access" {
  bucket = aws_s3_bucket.images.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

# resource "aws_s3_bucket_policy" "allow_access_from_lambdas" {
#   bucket = aws_s3_bucket.images.id
#   policy = data.aws_iam_policy_document.allow_lambda_access.json
# }

# data "aws_iam_policy_document" "allow_lambda_access" {
#   statement {
#     effect = "Allow"
#     actions = [
#       "s3:GetObject",
#       "s3:PutObject",
#       "s3:ListBucket",
#     ]

#     resources = [
#       aws_s3_bucket.images.arn,
#       "${aws_s3_bucket.images.arn}/*"
#     ]
#   }
# }

