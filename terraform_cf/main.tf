provider "cloudflare" {}

resource "cloudflare_dns_record" "river" {
  zone_id = var.cloudflare_zone_id
  name    = var.river_dns_name
  content = var.river_dns_content
  type    = "CNAME"
  proxied = false
  ttl     = 60
}
