job "redis" {
  type = "service"

  group "db" {
    count = 1
    network {
      port "redis" {
        to = 6379
      }
    }

    service {
      name     = "redis-svc"
      port     = "redis"
      provider = "nomad"
    }

    task "redis-task" {
      driver = "docker"

      config {
        image = "redis:7-alpine"
        ports = ["redis"]
      }
    }
  }
}