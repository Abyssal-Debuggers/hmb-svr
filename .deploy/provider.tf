locals {
  keycloak_port_forward = "50000"
}

provider "null" {}

provider "kubernetes" {
  config_path    = var.kubernetes_config_path
  config_context = var.kubernetes_config_context
}