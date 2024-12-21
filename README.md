# k8s secure verb-noun app

At core, this is a simple app composed of:
- a "verb" service that generates random verbs
- a "noun" service that generates random nouns
- an "aggregator" service that combines and returns the two to the user

The app is designed to be deployed on a Kubernetes cluster. It was developed on a **kind cluster**, but should work on any Kubernetes cluster.

It will be assumed that the user has **kind**, **kubectl** and **helm** installed.

## Getting started

To create the cluster, deploy the app and its security, run the following commands:

```bash
make run
```

If you already have a cluster up and running, you can deploy the app and its security by running the following commands:

```bash
make deploy
```

You can also run the following commands to clean up the cluster:

```bash
make destroy
```

## Testing features

### The app

The app will be accessible through the Ingress at [http://aggregator.private](http://aggregator.private). Don't forget to add the following line to your `/etc/hosts` file:

```bash
127.0.0.1 aggregator.private
```

The aggregator simply combines the results of the verb and noun services and returns them without any formating.

### Kyverno

The app is secured by Kyverno. The policies are defined in the `kyverno` directory. The policies are as follows:
- 3 admission policies:
  - `allowed-image-repos`: only allows images from the `ghcr.io/remi-espie/sentences` repository
  - `disallow-privileged-containers`: disallows privileged containers
  - `require-resource-limits`: requires resource limits to be set on all containers
- 2 mutation policies:
  - `add-resource-requests`: adds resource requests to all containers
  - `add-security-context`: adds security context to all containers

The Ingress, Falco and Istio are started before adding Kyverno policies, so they are not affected by them.

Each policy can be tested by deploying the app and trying to deploy a pod that violates the policy. For example, to test the `allowed-image-repos` policy, you can run the following command:

```bash
kubectl run test --image=nginx
```

### Falco

Falco was installed following the best practices from the [Falco documentation](https://falco.org/docs/getting-started/falco-kubernetes-quickstart/). The rules are the ones set by default.

To trigger a Falco alert, you can run the following command:

```bash
kubectl exec -it $(kubectl get pods --selector=app=verbs -o name -n sentence) -n sentence -- cat /etc/shadow
```
 and then check the Falco logs:
```bash
kubectl logs -l app.kubernetes.io/name=falco -n falco -c falco | grep Warning
```

### Istio

Service mesh with mTLS was installed with Istio. The app is secured by Istio, and only the `aggregator` service is accessible from the Ingress. The `verb` and `noun` services are only accessible from the `aggregator` service.

If not installed, Istio will be installed on the local machine.

To test the Istio security, you can run the following command:

```bash
kubectl exec -it $(kubectl get pods --selector=app=aggregator -o name -n sentence) -n sentence -- curl -v http://verb:8081
```

