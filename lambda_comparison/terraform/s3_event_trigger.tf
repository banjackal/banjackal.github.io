resource "aws_s3_bucket_notification" "bucket_notifications" {
  bucket = aws_s3_bucket.images.id

  lambda_function {
    lambda_function_arn = module.python_lambda_function.lambda_function_arn
    events              = ["s3:ObjectCreated:*"]
    filter_prefix       = "upload_for_python/"
  }

  lambda_function {
    lambda_function_arn = module.rust_lambda_function.lambda_function_arn
    events              = ["s3:ObjectCreated:*"]
    filter_prefix       = "upload_for_rust/"
  }
}
