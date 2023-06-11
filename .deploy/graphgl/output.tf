output "namespace" {
  value = kubernetes_namespace.graphql.metadata[0].name
}
