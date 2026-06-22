variable "cloudflare_zone_id" {
  description = "Cloudflare zone ID for duxca.com"
  type        = string
}

variable "river_dns_name" {
  description = "DNS record name for the River Cloud Run domain mapping"
  type        = string
  default     = "river.duxca.com"
}

variable "river_dns_content" {
  description = "DNS CNAME target for the River Cloud Run domain mapping"
  type        = string
  default     = "ghs.googlehosted.com"
}
