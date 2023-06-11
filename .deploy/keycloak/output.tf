output "namespace" {
  value = kubernetes_namespace.keycloak.metadata[0].name
}

output "service" {
  value = kubernetes_service.keycloak.metadata[0].name
}

output "service_port" {
  value = local.port
}

output "service_domain" {
  value = format(
    "%s.%s.svc.cluster.local",
    kubernetes_service.keycloak.metadata[0].name,
    kubernetes_namespace.keycloak.metadata[0].name,
  )
}


output "url" {
  value = format(
    "http://%s.%s.svc.cluster.local",
    kubernetes_service.keycloak.metadata[0].name,
    kubernetes_namespace.keycloak.metadata[0].name,
  )
}