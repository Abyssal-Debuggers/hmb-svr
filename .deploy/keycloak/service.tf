locals {
  port = "80"
}

resource "kubernetes_namespace" "keycloak" {
  metadata {
    name = "keycloak"
  }
}

resource "kubernetes_secret" "keycloak-admin" {
  metadata {
    name      = "keycloak-admin"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
  }
  type = "Opaque"
  data = {
    username = var.keycloak_admin_username
    password = var.keycloak_admin_password
  }
}

resource "kubernetes_secret" "postgres-admin" {
  metadata {
    name      = "postgres-admin"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
  }
  type = "Opaque"
  data = {
    host     = var.postgres_host
    port     = var.postgres_port
    database = var.postgres_database
    schema   = var.postgres_schema
    username = var.postgres_username
    password = var.postgres_password
  }
}

resource "kubernetes_deployment" "keycloak" {
  metadata {
    name      = "keycloak-admin"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
    labels    = {
      app = "keycloak"
    }
  }
  spec {
    replicas = "1"
    selector {
      match_labels = {
        app = "keycloak"
      }
    }
    template {
      metadata {
        labels = {
          app = "keycloak"
        }
      }
      spec {
        container {
          name  = "server"
          image = "bitnami/keycloak:21.1.1"

          dynamic "env" {
            for_each = {
              "KEYCLOAK_ADMIN_USER"        = ["keycloak-admin", "username"],
              "KEYCLOAK_ADMIN_PASSWORD"    = ["keycloak-admin", "password"],
              "KEYCLOAK_DATABASE_HOST"     = ["postgres-admin", "host"],
              "KEYCLOAK_DATABASE_PORT"     = ["postgres-admin", "port"],
              "KEYCLOAK_DATABASE_NAME"     = ["postgres-admin", "database"],
              "KEYCLOAK_DATABASE_SCHEMA"   = ["postgres-admin", "schema"],
              "KEYCLOAK_DATABASE_USER"     = ["postgres-admin", "username"],
              "KEYCLOAK_DATABASE_PASSWORD" = ["postgres-admin", "password"],

            }
            content {
              name = env.key
              value_from {
                secret_key_ref {
                  name = env.value[0]
                  key  = env.value[1]
                }
              }
            }
          }

          dynamic "env" {
            for_each = {
              "KEYCLOAK_DATABASE_VENDOR" = "postgresql"
            }
            content {
              name  = env.key
              value = env.value
            }
          }

          resources {
            requests = {
              memory = "256Mi"
              cpu    = "500m"
            }
            limits = {
              memory = "512Mi"
              cpu    = "1000m"
            }
          }
          port {
            name           = "web"
            container_port = 8080
          }
          readiness_probe {
            http_get {
              path = "/realms/master"
              port = "8080"
            }
          }
        }
      }
    }
  }
  lifecycle {
    ignore_changes = [
      metadata[0].annotations["autopilot.gke.io/resource-adjustment"],
      spec[0].template[0].spec[0].container[0].resources[0].requests,
      spec[0].template[0].spec[0].container[0].resources[0].limits,
      spec[0].template[0].spec[0].container[0].security_context,
      spec[0].template[0].spec[0].security_context,
      spec[0].template[0].spec[0].toleration,
    ]
  }
}

resource "kubernetes_service" "keycloak" {
  metadata {
    name      = "keycloak"
    namespace = kubernetes_namespace.keycloak.metadata[0].name
    labels    = {
      app = "keycloak"
    }
  }
  spec {
    selector = {
      app = "keycloak"
    }
    port {
      name        = "web"
      port        = "80"
      target_port = "8080"
    }
  }
  lifecycle {
    ignore_changes = [
      metadata[0].annotations["cloud.google.com/neg"],
    ]
  }
}

