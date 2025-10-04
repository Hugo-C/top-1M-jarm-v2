agent {
  policy = "read"
}

node {
  policy = "read"
}

namespace "*" {
  policy = "read"
  capabilities = ["submit-job", "dispatch-job", "read-logs", "read-fs", "alloc-exec", "scale-job"]

  variables {
    path "nomad/*" {
      capabilities = ["read", "write"]
    }
  }
}