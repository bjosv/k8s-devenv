# kind internals

## Enter master container
```
docker exec -ti kind-control-plane /bin/bash
```

## Get containers in master (enter containers first)
```
ctr -n k8s.io images list
```
