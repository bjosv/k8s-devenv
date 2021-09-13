# General

## Add label to all nodes
for node in $(kubectl get nodes --output=jsonpath={.items..metadata.name}); do kubectl label nodes $node topology.kubernetes.io/zone=local; done
k get nodes --show-labels=true

## Remove label from all nodes
for node in $(kubectl get nodes --output=jsonpath={.items..metadata.name}); do kubectl label nodes $node topology.kubernetes.io/zone-; done
