removed {
  from = google_storage_bucket.litestream_bucket

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_storage_bucket_iam_member.bucket_object_admin

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_storage_bucket_iam_member.bucket_legacy_reader

  lifecycle {
    destroy = false
  }
}

removed {
  from = google_storage_bucket_iam_member.bucket_legacy_writer

  lifecycle {
    destroy = false
  }
}
