# Moviestreamer helm chart

This document will explain how to install this helm chart to a cluster.

## Prerequisites

### Enable microk8s features

The following features should be enabled on microk8s

- dns (TODO)
- ha-cluster (TODO)
- helm3 (install the helm template)
- ingress (entry point for services)
- registry (push dockers to registry)
- storage (used by postgres database service)

### Installing cert-manager (only on production, locally we can use http)

See [this tutorial](https://www.cloudsavvyit.com/14069/how-to-install-kubernetes-cert-manager-and-configure-lets-encrypt/)
After reading [this thread on reddit](https://www.reddit.com/r/kubernetes/comments/g3z5sp/microk8s_with_certmanager_and_letsecncrypt/), I realized that I also have to add `kubernetes.io/tls-acme: "true"` to the annotations of my Ingress.

### Adding the container registry secret (only on production, locally the local registry of microk8s is used)

Before pulling the containers, the registry secret should be set. This can be done in the following manner ([source: kubernetes](https://kubernetes.io/docs/tasks/configure-pod-container/pull-image-private-registry/))

```bash
kubectl create secret docker-registry regcred --docker-server=registry.willemserruys.com --docker-username=XXX --docker-password=XXX --docker-email=serruys.willem@hotmail.com -n moviestreamer
```

The secret can be inspected as follows

```bash
kubectl get secret regcred --output="jsonpath={.data.\.dockerconfigjson}" | base64 --decode
```

And an example pod

```yml
apiVersion: v1
kind: Pod
metadata:
  name: private-reg
spec:
  containers:
  - name: private-reg-container
    image: <your-private-image>
  imagePullSecrets:
  - name: regcred
```

### Build and push container images to local or production registry

See the `scripts` folder under the root folder of this repo

### Execute permissions to scripts

Before executing `install.sh` or `uninstall.sh`, you should give the scripts executing permissions.
This can be done the following way

```bash
chmod +x install-local.sh
chmod +x uninstall.sh
...
```

### Add secret to kubernetes namespace

Execute the `add-secrets-local.sh` script in the `helm/scripts` folder

### Configure hostnames in hosts file (only when running with made up hostnames locally)

Add the hostnames to the `etc/hosts` file, redirecting to `127.0.0.1`

### Run install script

Execute the `install-local.sh` script in the `helm/scripts` folder
