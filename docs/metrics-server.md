# metrics-server

## Install
> minikube (gives v0.4.2)
minikube addons enable metrics-server

> kind or minikube (gives v0.5.0)
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml
kubectl edit deployment metrics-server -n kube-system
  Add following args to the metrics-server container:
  --kubelet-insecure-tls

> panic: Metric-resolution should be a time duration at least 10s, but value 5s provided

## Enable verbose logging (minikube)
kubectl edit deployment metrics-server -n kube-system
> Set log level, add:
- -v=10
k logs -n kube-system pod/metrics-server-54f97587df-fl9pk

## Get metrics (minikube)
kubectl get --raw /apis/metrics.k8s.io/v1beta1/namespaces/kube-system/pods/kube-scheduler-minikube | jq

## Code
https://github.com/kubernetes-sigs/metrics-server
ec ~/go/src/sigs.k8s.io/metrics-server/cmd/metrics-server/metrics-server.go

## Build
docker build -t k8s.gcr.io/metrics-server/metrics-server:v0.4.2 .
