# minikube

## Start
minikube start --nodes 3

## Upload image
minikube cache add alpine:latest
minikube cache reload
minikube cache list

## Upload files
scp -i $(minikube ssh-key) <local-path> docker@$(minikube ip):<remote-path>

## Start metrics server
minikube addons enable metrics-server
> or install manually to get specific specific version, see [metrics-server.md](see metrics-server.md)
