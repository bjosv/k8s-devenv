# Build minikube

## Devroot image
```
make buildroot-image
```

## Build iso
```
make out/minikube.iso
docker run --rm --workdir /mnt --volume /home/bjorn/git/minikube:/mnt  \
	--user 1000:1000 --env HOME=/tmp --env IN_DOCKER=1 \
	gcr.io/k8s-minikube/buildroot-image /usr/bin/make out/minikube.iso
````

## Run iso
```
./out/minikube start --iso-url=file://$(pwd)/out/minikube.iso
```

## Links
https://nanikjava.github.io/post/insideminikubeiso/

https://stackoverflow.com/questions/54747069/how-can-i-install-use-tcpdump-on-minikube
