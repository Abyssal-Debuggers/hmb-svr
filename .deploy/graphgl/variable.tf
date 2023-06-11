variable "server_port" { type = string }
variable "server_ip" { type = string }

variable "keycloak_url" { type = string }
variable "keycloak_realm" { type = string }
variable "keycloak_username" { type = string }
variable "keycloak_password" { type = string }

variable "postgres_host" { type = string }
variable "postgres_port" { type = string }
variable "postgres_database" { type = string }
variable "postgres_username" { type = string }
variable "postgres_password" { type = string }


variable "graphql_image" { type = string }