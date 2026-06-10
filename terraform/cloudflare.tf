provider "cloudflare" {}

resource "cloudflare_dns_record" "river" {
  zone_id = var.cloudflare_zone_id
  name    = "river.duxca.com"
  content = "ghs.googlehosted.com"
  type    = "CNAME"
  proxied = false
  ttl     = 60
}
