module "keycloak" {
  source = "./keycloak"

  keycloak_admin_username = var.keycloak_admin_username
  keycloak_admin_password = var.keycloak_admin_password

  postgres_host     = var.postgres_host
  postgres_port     = var.postgres_port
  postgres_username = var.postgres_username
  postgres_password = var.postgres_password
  postgres_database = var.postgres_database
  postgres_schema   = "keycloak"
}

module "graphgl" {
  source = "./graphgl"

  server_port = 80
  server_ip   = null

  keycloak_url      = module.keycloak.url
  keycloak_realm    = "hmb-auth"
  keycloak_username = var.keycloak_admin_username
  keycloak_password = var.keycloak_admin_password

  postgres_host     = var.postgres_host
  postgres_port     = var.postgres_port
  postgres_database = var.postgres_database
  postgres_username = var.postgres_username
  postgres_password = var.postgres_password

  graphql_image = var.graphql_image
}

module "gke-setup" {
  source     = "./gke-setup"
  count      = var.gke_setup ? 1 : 0
  namespaces = [
    module.keycloak.namespace,
    module.graphgl.namespace
  ]
  domains = [
    "auth.hmbgaq.org",
    "graphgl.hmbgaq.org",
    "api.hmbgaq.org",
    "www.hmbgaq.org",
  ]
}