data "aws_iam_policy_document" "lambda_policy" {
  statement {
    effect = "Allow"
    actions = [
      "s3:GetObject",
      "s3:ListBucket",
      "s3:PutObject",
    ]
    resources = [
      aws_s3_bucket.images.arn,
      "${aws_s3_bucket.images.arn}/*"
    ]
  }

  statement {
    effect = "Allow"
    actions = [
      "dynamodb:GetItem",
      "dynamodb:BatchGetItem",
      "dynamodb:Query",
      "dynamodb:PutItem",
      "dynamodb:UpdateItem",
      "dynamodb:DeleteItem",
      "dynamodb:BatchWriteItem"
    ]
    resources = [
      module.dynamodb_table.dynamodb_table_arn
    ]
  }
}

module "python_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  publish = true

  function_name = "python_image_processor"
  description   = "Creates thumbnails of uploaded S3 images"
  handler       = "lambda_function.lambda_handler"
  runtime       = "python3.10"
  source_path   = "../python/"

  allowed_triggers = {
    S3Events = {
      principal  = "s3.amazonaws.com"
      source_arn = "arn:aws:s3:::*"
    }
  }

  attach_policy_json = true
  policy_json        = data.aws_iam_policy_document.lambda_policy.json

  memory_size = 128
  timeout     = 5
}

module "rust_lambda_function" {
  source = "terraform-aws-modules/lambda/aws"

  publish = true

  function_name          = "rust_image_processor"
  description            = "Creates thumbnails of uploaded S3 images"
  handler                = "bootstrap"
  runtime                = "provided.al2"
  architectures          = ["arm64"]
  create_package         = false
  local_existing_package = "../rust-image-processor/target/lambda/rust-image-processor/bootstrap.zip"

  allowed_triggers = {
    S3Events = {
      principal  = "s3.amazonaws.com"
      source_arn = "arn:aws:s3:::*"
    }
  }

  attach_policy_json = true
  policy_json        = data.aws_iam_policy_document.lambda_policy.json

  memory_size = 128
  timeout     = 5
}


