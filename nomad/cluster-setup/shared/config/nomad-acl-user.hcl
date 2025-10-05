agent {
  policy = "read"
}

node {
  policy = "write"
}

namespace "*" {
  policy = "read"
  capabilities = ["submit-job", "dispatch-job", "read-logs", "read-fs", "alloc-exec", "scale-job"]

  variables {
    path "nomad/*" {
      capabilities = ["write"]
    }
  }
}