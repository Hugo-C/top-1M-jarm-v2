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

## Scale the jobs

By default, the worker has 2 instances and scheduler and uploader 0.
Start the scheduler with:
```shell
nomad job scale compute scheduler 1
```

Once it has completed the schedule of the 1M hosts, **turn it down with**:
```shell
```shell
nomad job scale compute scheduler 0
```

Then start the uploader with
```shell
nomad job scale compute uploader 1
```

Finally, modulate workers with 
```shell
nomad job scale compute worker X
```