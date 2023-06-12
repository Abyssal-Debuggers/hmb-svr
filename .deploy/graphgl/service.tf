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
          name    = "server"
          image   = var.graphql_image
          command = ["/usr/app/application", "--config", "/usr/config/config.json"]
          volume_mount {
            mount_path = "/usr/config"
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


resource "kubernetes_service" "graphql" {
  metadata {
    name      = "graphql"
    namespace = kubernetes_namespace.graphql.metadata[0].name
  }
  spec {
    selector = {
      app = kubernetes_deployment.graphql.spec[0].template[0].metadata[0].labels.app
    }
    port {
      port = kubernetes_deployment.graphql.spec[0].template[0].spec[0].container[0].port[0].container_port
      name = kubernetes_deployment.graphql.spec[0].template[0].spec[0].container[0].port[0].name
    }
  }
  lifecycle {
    ignore_changes = [
      metadata[0].annotations["cloud.google.com/neg"],
      metadata[0].annotations["cloud.google.com/neg-status"],
    ]
  }
}


resource "kubernetes_manifest" "graphql-ingress" {
  manifest = {
    apiVersion = "networking.k8s.io/v1"
    kind       = "Ingress"
    metadata   = {
      name        = "graphql-ingress"
      namespace   = kubernetes_namespace.graphql.metadata[0].name
      annotations = {
        "kubernetes.io/ingress.class"                 = "gce"
        "kubernetes.io/ingress.allow-http"            = "true"
        "kubernetes.io/ingress.global-static-ip-name" = var.static_ip_name
        "networking.gke.io/managed-certificates"      = var.managed_certificates_name
      }
    }
    spec = {
      rules = [
        {
          host = "graph.hmbgaq.org"
          http = {
            paths = [
              {
                path     = "/"
                pathType = "Prefix"
                backend  = {
                  service = {
                    name = kubernetes_service.graphql.metadata[0].name
                    port = {
                      number = kubernetes_service.graphql.spec[0].port[0].port
                    }
                  }
                }
              }
            ]
          }
        }
      ]
    }
  }
}