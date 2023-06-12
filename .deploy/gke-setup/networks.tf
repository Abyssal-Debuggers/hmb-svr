locals {
  name = "hmb-ssl"
}
resource "kubernetes_manifest" "managed_certificate" {
  for_each = toset(var.namespaces)

  manifest = {
    apiVersion = "networking.gke.io/v1"
    kind       = "ManagedCertificate"
    metadata   = {
      name      = "hmb-ssl"
      namespace = each.key
    }
    spec = {
      domains = var.domains
    }
  }
}