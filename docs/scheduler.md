# Scheduler

## Code
https://github.com/kubernetes/kubernetes/tree/master/pkg/scheduler

## Print scheduler config
docker exec -it kind-control-plane cat /etc/kubernetes/manifests/kube-scheduler.yaml

## Update log level
docker exec -it kind-control-plane sed -i 's/--port=0/--port=0\n    - -v=10/' /etc/kubernetes/manifests/kube-scheduler.yaml

## Get scheduler log
k logs kube-scheduler-kind-control-plane -n kube-system



## Links
### Own scheduler
https://blog.searce.com/create-custom-scheduler-on-gke-for-pod-spreading-a23c1641a840

### How the scheduler iterates over Nodes
https://kubernetes.io/docs/concepts/scheduling-eviction/scheduler-perf-tuning/#how-the-scheduler-iterates-over-nodes
