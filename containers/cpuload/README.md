# cpuload

## Build
docker build -t bjosv/cpuload:latest .
docker push bjosv/cpuload:latest

## Run
docker run -e HOG_SECONDS=5 bjosv/cpuload

kubectl apply -f configs/cpuload.yaml
