terraform {
  backend "gcs" {
    bucket = "mitoma-org-tfstate"
    prefix = "terraform/state"
  }
}
