module "dynamodb_table" {
  source = "terraform-aws-modules/dynamodb-table/aws"

  name     = "image_metadata"
  hash_key = "id"

  attributes = [
    {
      name = "id"
      type = "S"
    }
  ]
}
