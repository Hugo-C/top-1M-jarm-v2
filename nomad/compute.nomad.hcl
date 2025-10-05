job "compute" {
  type = "batch"

  group "scheduler" {
    count = 0

    task "scheduler-task" {
      template {
        data        = <<EOH
{{ range nomadService "redis-svc" }}
REDIS_HOST={{ .Address }}
REDIS_PORT={{ .Port }}
RUST_BACKTRACE=1
SENTRY_DSN={{ with nomadVar "nomad/jobs" }}{{ .sentry_dsn }}{{ end }}
{{ end }}
EOH
        destination = "local/env.txt"
        env         = true
      }

      driver = "docker"

      config {
        image   = "hugocker/top-1m-jarm-v2"
        command = "scheduler"
      }
    }
  }

  group "worker" {
    count = 0
    ephemeral_disk {
      size = 100 # MB
    }

    task "worker-task" {
      template {
        data        = <<EOH
{{ range nomadService "redis-svc" }}
REDIS_HOST={{ .Address }}
REDIS_PORT={{ .Port }}
RUST_BACKTRACE=1
SENTRY_DSN={{ with nomadVar "nomad/jobs" }}{{ .sentry_dsn }}{{ end }}
{{ end }}
EOH
        destination = "local/env.txt"
        env         = true
      }

      driver = "docker"

      config {
        image   = "hugocker/top-1m-jarm-v2"
        command = "worker"
      }

      resources {
        cpu = 128 # MHz
        memory = 32 # MB
      }
      logs {
        max_files     = 2
        max_file_size = 10 # MB
      }
    }
  }

  group "uploader" {
    count = 0

    task "uploader-task" {
      template {
        data        = <<EOH
{{ range nomadService "redis-svc" }}
REDIS_HOST={{ .Address }}
REDIS_PORT={{ .Port }}
RUST_BACKTRACE=1
SENTRY_DSN={{ with nomadVar "nomad/jobs" }}{{ .sentry_dsn }}{{ end }}
AWS_ACCESS_KEY_ID={{ with nomadVar "nomad/jobs/compute/uploader/uploader-task" }}{{ .aws_access_key_id }}{{ end }}
AWS_SECRET_ACCESS_KEY={{ with nomadVar "nomad/jobs/compute/uploader/uploader-task" }}{{ .aws_secret_access_key }}{{ end }}
{{ end }}
EOH
        destination = "local/env.txt"
        env         = true
      }

      driver = "docker"

      config {
        image   = "hugocker/top-1m-jarm-v2"
        command = "uploader"
      }
    }
  }
}