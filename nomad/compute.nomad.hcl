job "compute" {
  type = "batch"

  group "scheduler" {
    count = 1

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
        image   = "top-1m-jarm-v2:nomad"
        command = "scheduler"
      }
    }
  }

  group "worker" {
    count = 2

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
        image   = "top-1m-jarm-v2:nomad"
        command = "worker"
      }

      resources {
        cpu = 500 # MHz
        memory = 128 # MB
      }
    }
  }

  group "uploader" {
    count = 1

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
        image   = "top-1m-jarm-v2:nomad"
        command = "uploader"
      }
    }
  }
}