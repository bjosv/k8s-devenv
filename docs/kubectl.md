# kubectl

## Code
https://github.com/kubernetes/kubernetes/tree/master/cmd/kubectl
https://github.com/kubernetes/kubectl


## Build
cd $GOPATH/src/k8s.io/kubernetes/
KUBE_BUILD_PLATFORMS=linux/amd64 make WHAT=cmd/kubectl
> Run
_output/local/bin/linux/amd64/kubectl
