provider "null" {}

provider "google" {
  project = var.gcp_project
  region  = var.gcp_region
  zone    = var.gcp_zone
}

provider "kubernetes" {
  config_path    = var.kubernetes_config_path
  config_context = var.kubernetes_config_context
}