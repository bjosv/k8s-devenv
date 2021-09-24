# kubelet

## Build kubelet
KUBE_BUILD_PLATFORMS=linux/amd64 make WHAT=cmd/kubelet
scp -i $(minikube ssh-key --node minikube-m03) _output/local/bin/linux/amd64/kubelet docker@$(minikube ip --node minikube-m03):~/
minikube ssh -n minikube-m03 sudo chown root.root kubelet
minikube ssh -n minikube-m03 sudo mv kubelet /var/lib/minikube/binaries/v1.21.2/kubelet
minikube ssh -n minikube-m03 sudo systemctl restart kubelet
minikube ssh -n minikube-m03
sudo journalctl -u kubelet.service | grep BJOSV

## Get kubelet endpoint on node
kubectl get node minikube-m02 -o json | jq '.status.daemonEndpoints'
  "kubeletEndpoint": "Port": 10250

## Update config in minikube (--v=10)
sudo vi /etc/systemd/system/kubelet.service.d/10-kubeadm.conf
sudo systemctl daemon-reload
sudo systemctl restart kubelet

## Get logs (minikube)
minikube ssh -n minikube-m02
sudo journalctl -u kubelet.service
- or -
minikube logs --node=minikube-m02

## Links
### Kubelet process args
https://kubernetes.io/docs/reference/command-line-tools-reference/kubelet/

https://www.deepnetwork.com/blog/2020/01/13/kubelet-api.html
