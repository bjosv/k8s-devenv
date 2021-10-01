# kubelet

## Build kubelet
cd $GOPATH/src/k8s.io/kubernetes/
KUBE_BUILD_PLATFORMS=linux/amd64 make WHAT=cmd/kubelet

> minikube
scp -i $(minikube ssh-key --node minikube-m03) _output/local/bin/linux/amd64/kubelet docker@$(minikube ip --node minikube-m03):~/
minikube ssh -n minikube-m03 sudo chown root.root kubelet
minikube ssh -n minikube-m03 sudo mv kubelet /var/lib/minikube/binaries/v1.21.2/kubelet
minikube ssh -n minikube-m03 'sudo journalctl --rotate && sudo journalctl --vacuum-time=1s'
minikube ssh -n minikube-m03 sudo systemctl restart kubelet
minikube ssh -n minikube-m03 'sudo journalctl -u kubelet.service | grep BJOSV'
> kind
docker cp _output/local/bin/linux/amd64/kubelet kind-worker:/usr/bin/kubelet
docker exec -it kind-worker chown root.root /usr/bin/kubelet
docker exec -it kind-worker sh -c 'journalctl --rotate && journalctl --vacuum-time=1s'
docker exec -it kind-worker systemctl restart kubelet
docker exec -it kind-worker journalctl -u kubelet | grep BJOSV

## Get kubelet endpoint on node
kubectl get node minikube-m02 -o json | jq '.status.daemonEndpoints'
  "kubeletEndpoint": "Port": 10250

## Update config in minikube (--v=10)
minikube ssh -n minikube-m03
sudo vi /etc/systemd/system/kubelet.service.d/10-kubeadm.conf
sudo systemctl daemon-reload
sudo systemctl restart kubelet

## Update housekeeping interval (hack!)
--housekeeping-interval=5s

## Get logs (minikube)
minikube ssh -n minikube-m02
sudo journalctl -u kubelet.service
- or -
minikube logs --node=minikube-m02

## Clear logs
minikube ssh -n minikube-m03 'sudo journalctl --rotate && sudo journalctl --vacuum-time=1s'

## Enable access to kubelets metrics/resource endpoint
k apply -f configs/cadvisor.yaml
TOKEN=$(kubectl -n kube-system get secrets monitoring-secret-token -ojsonpath='{.data.token}' | base64 -d)
> minikube
minikube ssh -n minikube-m03 -- curl -k --header \"Authorization: Bearer $TOKEN\" https://127.0.0.1:10250/metrics/resource | grep container_cpu_usage_seconds_total
or
minikube ssh -n minikube -- curl -k --header \"Authorization: Bearer $TOKEN\" 'https://minikube:10250/stats/summary?only_cpu_and_memory=true'
minikube ssh -n minikube -- curl -k --header \"Authorization: Bearer $TOKEN\" 'https://minikube:10250/stats/summary?only_cpu_and_memory=true' | jq '.pods[] | select(.podRef.name == "kube-controller-manager-minikube").cpu'
> kind
docker exec -it kind-worker curl -k --header 'Authorization: Bearer '$TOKEN 'https://kind-worker:10250/stats/summary?only_cpu_and_memory=true' | jq '.pods[] | select(.podRef.name == "cpuload").cpu'

## Links
### Kubelet process args
https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/
https://www.deepnetwork.com/blog/2020/01/13/kubelet-api.html

## Code

### Stats: /metrics/resource
Code: pkg/kubelet/server/server.go
k8s.io/kubernetes/pkg/kubelet/server/stats/summary.go

### Stats: /stats/summary (used by metrics-server 0.3.6)
pkg/kubelet/server/stats/handler.go    handleSummary()
pkg/kubelet/server/stats/summary.go    summaryProvider.GetCPUAndMemoryStats()
                                       <provider>.ListPodCPUAndMemoryStats()
kubelet is a provider, uses StatsProvider
// UsingLegacyCadvisorStats returns true if container stats are provided by cadvisor instead of through the CRI.
- pkg/kubelet/stats/cadvisor_stats_provider.go   getCadvisorContainerInfo()
- pkg/kubelet/stats/cri_stats_provider.go

### cAdvisor
#### Stats provider
  kubernetes/pkg/kubelet/cadvisor/cadvisor_linux.go   ContainerInfoV2()

vendor/github.com/google/cadvisor/manager/manager.go
GetContainerInfoV2()

vendor/github.com/google/cadvisor/manager/container.go
- cAdvisor adds supervision of found containers, running parallell to the statistic provider endpoint.
- A housekeeping timer checks cpu/mem stats at a calculated interval.
  If stats have not changed since previous reading, the interval will be increased until next reading.
  Minimum interval ~= 10s (Default) * 1 (jitter) = 10s
  Maximum interval ~= 15s (Max)     * 2 (jitter) = 30s

- The default interval is normally hardcoded to 10s (but it will be multiplied with a random jitter, times 1-2)
  The value can be configured by starting the kubelet with following added flags:
    --housekeeping-interval=5s
  !! But it seems to be a hidden parameter hack for e2e tests...

- The maximum interval (15s, which will be multiplied with a random jitter, times 1-2)
  is not configurable.


#### Stats "fetcher"
Data placed in m.containers

kubernetes/vendor/github.com/google/cadvisor/manager/container.go
- housekeepingTick
- cd.updateStats()
- cd.handler.GetStats()
vendor/github.com/google/cadvisor/container/containerd/handler.go
- GetStats()
vendor/github.com/google/cadvisor/container/libcontainer/handler.go
- GetStats()

vendor/github.com/opencontainers/runc/libcontainer/cgroups/fs2/fs2.go
vendor/github.com/opencontainers/runc/libcontainer/cgroups/fs2/cpu.go

#### Container management
CPU:
k8s.io/kubernetes/pkg/kubelet/cm/cpumanager/cpu_manager.go
