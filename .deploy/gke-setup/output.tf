output "managed_certificate" {
  value = {
    for ns in var.namespaces : ns
    => kubernetes_manifest.managed_certificate[ns].manifest.metadata.name
  }
}