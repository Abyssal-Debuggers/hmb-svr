module "graphgl" {
  source = "./graphgl"

  server_port = 80
  server_ip   = null

  keycloak_url      = var.keycloak_url
  keycloak_realm    = "hmb-auth"
  keycloak_username = var.keycloak_admin_username
  keycloak_password = var.keycloak_admin_password

  postgres_host     = var.postgres_host
  postgres_port     = var.postgres_port
  postgres_database = var.postgres_database
  postgres_username = var.postgres_username
  postgres_password = var.postgres_password

  graphql_image = var.graphql_image

  managed_certificates_name = module.gke-setup.managed_certificate_name
  static_ip_name            = var.graphql_static_ip_name
}

module "gke-setup" {
  source = "./gke-setup"
  namespaces = [
    module.graphgl.namespace
  ]
  domains = [
    "auth.hmbgaq.org",
    "graphgl.hmbgaq.org",
    "api.hmbgaq.org",
    "www.hmbgaq.org",
  ]
}
