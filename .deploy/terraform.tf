terraform {
  backend "pg" {

  }
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.21.1"
    }
    null = {
      source  = "hashicorp/null"
      version = "3.2.1"
    }
  }
}


