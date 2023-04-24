terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.64.0"
    }
  }
}

provider "aws" {
  region = "us-east-1"
  default_tags {
    tags = {
      Owner       = "Nick"
      Provisioner = "Terraform"
      Project     = "Rust v Python Lambda"
    }
  }
}


