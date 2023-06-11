resource "kubernetes_namespace" "graphql" {
  metadata {
    name = "graphql"
  }
}

resource "kubernetes_secret" "graphql-config" {
  metadata {
    name      = "graphql-config"
    namespace = kubernetes_namespace.graphql.metadata[0].name
  }
  type = "Opaque"
  data = {
    "config.json" = jsonencode({
      server = {
        port = var.server_port
        ip   = var.server_ip
      }
      keycloak = {
        url      = var.keycloak_url
        realm    = var.keycloak_realm
        username = var.keycloak_username
        password = var.keycloak_password
      }
      database = {
        host     = var.postgres_host
        port     = var.postgres_port
        username = var.postgres_username
        password = var.postgres_password
        database = var.postgres_database
      }
    })
  }
}

resource "kubernetes_deployment" "graphql" {
  metadata {
    name      = "graphql-server"
    namespace = kubernetes_namespace.graphql.metadata[0].name
    labels    = {
      app = "graphql"
    }
  }
  spec {
    replicas = "1"
    selector {
      match_labels = {
        app = "graphql"
      }
    }
    template {
      metadata {
        labels = {
          app = "graphql"
        }
      }
      spec {
        container {
          name  = "server"
          image = var.graphql_image
          volume_mount {
            mount_path = "/usr/app"
            name       = "config"
            read_only  = true
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
            container_port = var.server_port
          }
        }
        volume {
          name = "config"
          secret {
            secret_name = kubernetes_secret.graphql-config.metadata[0].name
          }
        }
      }
    }
  }
}
