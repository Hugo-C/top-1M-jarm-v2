job "uploader" {
  type = "batch"

  group "scheduler-worker" {
    count = 1

    task "uploader-task" {
            template {
        data        = <<EOH
{{ range nomadService "redis-svc" }}
REDIS_HOST={{ .Address }}
REDIS_PORT={{ .Port }}
RUST_BACKTRACE=1
SENTRY_DSN=XXX
AWS_ACCESS_KEY_ID=XXX
AWS_SECRET_ACCESS_KEY=XXX
{{ end }}
EOH
        destination = "local/env.txt"
        env         = true
      }

      driver = "docker"

      config {
        image = "top-1m-jarm-v2:nomad"
        command = "uploader"
      }
    }
  }
}