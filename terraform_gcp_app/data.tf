data "terraform_remote_state" "foundation" {
  backend = "gcs"

  config = {
    bucket = var.terraform_state_bucket
    prefix = var.foundation_state_prefix
  }
}
