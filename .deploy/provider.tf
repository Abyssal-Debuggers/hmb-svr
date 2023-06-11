locals {
  keycloak_port_forward = "50000"
}

provider "null" {}

provider "google" {
  project = "practice-make-perfect-379914"
  region  = "asia-northeast3"
  zone    = "asia-northeast3-a"
}

provider "kubernetes" {
  config_path    = var.kubernetes_config_path
  config_context = var.kubernetes_config_context
}