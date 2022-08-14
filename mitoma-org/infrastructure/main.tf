provider "google" {
  project = "mitoma-org"
  region  = "asia-northeast1"
  zone    = "asia-northeast1-c"
}

resource "google_storage_bucket" "tfstate" {
  name          = "mitoma-org-tfstate"
  force_destroy = false
  location      = "ASIA-NORTHEAST1"
  storage_class = "STANDARD"
  versioning {
    enabled = true
  }
}

resource "google_artifact_registry_repository" "mitoma-org-image" {
  location      = "asia-northeast1"
  repository_id = "mitoma-org"
  description   = "mitoma.org application image"
  format        = "DOCKER"
}

resource "google_cloud_run_service" "mitoma-org-app" {
  name     = "mitoma-org"
  location = "asia-northeast1"
  template {
    spec {
      containers {
        image = "asia-northeast1-docker.pkg.dev/mitoma-org/mitoma-org/app:8151b9fbd36d"
      }
    }
    metadata {
      annotations = {
        "autoscaling.knative.dev/maxScale" = "1"
        "autoscaling.knative.dev/minScale" = "0"
      }
    }
  }
  traffic {
    percent         = 100
    latest_revision = true
  }
}

data "google_iam_policy" "noauth" {
  binding {
    role = "roles/run.invoker"
    members = [
      "allUsers",
    ]
  }
}

resource "google_cloud_run_service_iam_policy" "noauth" {
  location = google_cloud_run_service.mitoma-org-app.location
  project  = google_cloud_run_service.mitoma-org-app.project
  service  = google_cloud_run_service.mitoma-org-app.name

  policy_data = data.google_iam_policy.noauth.policy_data
}

resource "google_cloud_run_domain_mapping" "mitoma-org" {
  location = "asia-northeast1"
  name     = "mitoma.org"
  metadata {
    namespace = "mitoma-org"
  }
  spec {
    route_name = google_cloud_run_service.mitoma-org-app.name
  }
}

resource "google_dns_managed_zone" "mitoma-org" {
  name        = "mitoma-org"
  dns_name    = "mitoma.org."
  description = "mitoma.org dns"
  dnssec_config {
    state = "off"
  }
}

resource "google_dns_record_set" "naked-domain-records" {
  for_each = {
    "A" = [
      for rr in google_cloud_run_domain_mapping.mitoma-org.status[0].resource_records :
      rr.rrdata if rr.type == "A"
    ]
    "AAAA" = [
      for rr in google_cloud_run_domain_mapping.mitoma-org.status[0].resource_records :
      rr.rrdata if rr.type == "AAAA"
    ]
  }
  name         = google_dns_managed_zone.mitoma-org.dns_name
  managed_zone = google_dns_managed_zone.mitoma-org.name

  type    = each.key
  rrdatas = each.value
  ttl     = 300
}
