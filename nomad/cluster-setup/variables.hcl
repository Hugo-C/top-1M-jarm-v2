# Packer variables (all are required)
region                    = "eu-west-3"

# Terraform variables (all are required)
ami                       = "XXX_REPLACE_ME_XXX"

# These variables will default to the values shown
# and do not need to be updated unless you want to
# change them
# allowlist_ip            = "0.0.0.0/0"
# name_prefix             = "nomad"
# server_instance_type    = "t3.micro"
# Set to 3 for HA cluster
# server_count            = "1"
# client_instance_type    = "t3.small"
# client_count            = "3"
