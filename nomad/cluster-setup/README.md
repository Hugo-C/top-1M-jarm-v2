Based on [Hashicorp doc](https://developer.hashicorp.com/nomad/tutorials/cluster-setup/cluster-setup-aws).  

Export aws credentials (can be found [here](https://us-east-1.console.aws.amazon.com/iam/home?region=eu-west-3#/security_credentials)) with:
```shell
export AWS_REGION=eu-west-3
export AWS_ACCESS_KEY_ID=XXX
export AWS_SECRET_ACCESS_KEY=XXX
```

Build the AMI image with:
```shell
packer init image.pkr.hcl
packer build -var-file=variables.hcl image.pkr.hcl
```

Then set its ID in [variables.hcl](variables.hcl), for example `
ami                          = "ami-0445eeea5e1406960"
`

Then deploy the cluster with Terraform:
```shell
terraform init
terraform apply -var-file=variables.hcl
```

Once all services are up, run the post-install script to pop the Nomad token from Consul
```shell
./post-setup.sh
export NOMAD_ADDR=$(terraform output -raw lb_address_consul_nomad):4646 &&  export NOMAD_TOKEN=$(cat nomad.token)
```

Nomad should then be ready to use:
```shell
nomad node status
```

⚠️ Don't forget to remove [the AMI](https://eu-west-3.console.aws.amazon.com/ec2/v2/home?region=eu-west-3#Images:visibility=owned-by-me;v=3;tag:Name=nomad-alb;sort=desc:creationDate) and [its S3 snapshot](https://eu-west-3.console.aws.amazon.com/ec2/home?region=eu-west-3#Snapshots:visibility=owned-by-me;v=3;tag:Name=nomad-alb;sort=desc:creationDate) afterward.