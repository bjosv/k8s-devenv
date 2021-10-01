# metrics-server

## Install
> minikube (gives v0.4.2)
minikube addons enable metrics-server

> kind or minikube (gives >= v0.5.0)
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/latest/download/components.yaml
kubectl edit deployment metrics-server -n kube-system
  Add following args to the metrics-server container:
  --kubelet-insecure-tls
  --metric-resolution=5s  (for 0.3.6)

> or given version
kubectl apply -f https://github.com/kubernetes-sigs/metrics-server/releases/download/v0.3.6/components.yaml

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

## Code details

### 0.5.0
scraper: metrics-server/pkg/scraper/client/resource/client.go

### 0.3.6
Collect metrics, and store in sink:  source.Collect()  metrics-server/pkg/manager/manager.go
Provider of container metrics:       sink.Receive()    metrics-server/pkg/provider/sink/sinkprov.go
Source                               source.Collect()  metrics-server/pkg/sources/manager.go
   https://minikube:10250/stats/summary?only_cpu_and_memory=true
Decodes:                             decodePodStats()  metrics-server/pkg/sources/summary/summary.go
      "cpu -> time", "memory -> time"
      "usageNanoCores"
      "WorkingSetBytes"

#### server.tick()
Each configured resolution:
  Scrape metrics -> store metrics
  MetricsBatch: Nodes and Pods
    CumulativeCpuUsed uint64: cumulative cpu used at Timestamp from the StartTime of container/node.
                      Unit: nano core * seconds.

?? PodMetricsPoint contains the metrics for some pod's containers.

#### scraper.collectNode()
kubeletClient.GetMetrics()
client.go: GetMetrics()
  GetMetrics get metrics from kubelet <node-addr>:port/metrics/resource endpoint
  - decode.go decodeBatch()
               parseContainerMemMetrics()
