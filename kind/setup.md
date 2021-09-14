# kind

## Start cluster (1 CP, 3 workers)

### Specific version
```
kind create cluster --image kindest/node:v1.16.9 --config 3node.yml
kind create cluster --image kindest/node:v1.17.5 --config 3node.yml
```
