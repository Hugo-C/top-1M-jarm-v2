job "scheduler" {
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
SENTRY_DSN=XXX
{{ end }}
EOH
        destination = "local/env.txt"
        env         = true
      }

      driver = "docker"

      config {
        image = "top-1m-jarm-v2:nomad"
        command = "scheduler"
      }
    }
  }
}