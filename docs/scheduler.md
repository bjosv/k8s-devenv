# Scheduler

## Code
https://github.com/kubernetes/kubernetes/tree/master/pkg/scheduler

## Print scheduler config
docker exec -it kind-control-plane cat /etc/kubernetes/manifests/kube-scheduler.yaml

## Update log level (will restart pod)
docker exec -it kind-control-plane sed -i 's/--port=0/--port=0\n    - -v=10/' /etc/kubernetes/manifests/kube-scheduler.yaml
k get pods -n kube-system

## Get scheduler log
k logs kube-scheduler-kind-control-plane -n kube-system


## Build changed scheduler container and use in kind
KUBE_BUILD_PLATFORMS="linux/amd64" make quick-release

### Check existing images on kind
docker exec -it kind-control-plane ctr -n k8s.io images list "name~=sched"

### Load image archive to kind
kind load image-archive $GOPATH/src/k8s.io/kubernetes/_output/release-images/amd64/kube-scheduler.tar

### Print current scheduler config
docker exec -it kind-control-plane cat /etc/kubernetes/manifests/kube-scheduler.yaml

### Update config/manifest
docker exec -it kind-control-plane sed -i 's|image: k8s.gcr.io/kube-scheduler:v1.21.1|image: k8s.gcr.io/kube-scheduler-amd64:v1.21.1-dirty|' /etc/kubernetes/manifests/kube-scheduler.yaml
kubectl -n kube-system get pods
kubectl logs kube-scheduler-kind-control-plane -n kube-system


## Links
### Own scheduler
https://blog.searce.com/create-custom-scheduler-on-gke-for-pod-spreading-a23c1641a840

### How the scheduler iterates over Nodes
https://kubernetes.io/docs/concepts/scheduling-eviction/scheduler-perf-tuning/#how-the-scheduler-iterates-over-nodes
