# Nomad

To use Nomad instead of `docker swarm` as an orchestrator follow the following steps:

## Start nomad:
```shell
sudo nomad agent -dev -bind 0.0.0.0 -network-interface='{{ GetDefaultInterfaces | attr "name" }}'
```

## Configure the required variables
```shell
nomad var put nomad/jobs sentry_dsn=XXX
nomad var put nomad/jobs/compute/uploader/uploader-task aws_access_key_id=XXX aws_secret_access_key=XXX
```

## Start the redis service
```shell
nomad job run nomad/redis.nomad.hcl
```

## Run the batch jobs

If using a development image, buid it `docker build -t top-1m-jarm-v2:nomad --pull --no-cache .` and use the tag in the hcl file.

Else simply run
```shell
nomad job run nomad/compute.nomad.hcl
```
