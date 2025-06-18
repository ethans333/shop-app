#!/bin/bash
set -e

IMAGE_NAME="ethans333/shop-app_api:latest"
KUBERNETES_DIR="./k8s"
LOCAL_PORT=8080

echo "[1/9] Pulling image: $IMAGE_NAME"
docker pull "$IMAGE_NAME"

echo "[2/9] Loading image into kind cluster"
kind load docker-image "$IMAGE_NAME"

echo "[3/9] Deleting old services"
kubectl delete service api-service --ignore-not-found
kubectl delete service postgres --ignore-not-found

echo "[4/9] Deleting old pods"
kubectl delete pods -l app=shop-app --ignore-not-found

echo "[5/9] Checking if local port $LOCAL_PORT is in use"
if lsof -i tcp:"$LOCAL_PORT" >/dev/null 2>&1; then
  PID=$(lsof -ti tcp:"$LOCAL_PORT")
  echo "Port $LOCAL_PORT is in use by PID $PID, killing process..."
  kill -9 "$PID"
else
  echo "Port $LOCAL_PORT is free."
fi

echo "[6/9] Applying Kubernetes manifests"
for entry in "$KUBERNETES_DIR"/*; do
  if [ -d "$entry" ]; then
    echo "Applying deployment and service in $entry"
    kubectl apply -f "$entry/deployment.yaml"
    kubectl apply -f "$entry/service.yaml"
  fi
done

echo "[7/9] Waiting for pods to be ready..."
kubectl wait --for=condition=ready pod -l app=shop-app --timeout=120s

echo "[8/9] Running migrator job"
kubectl delete job run-migrations --ignore-not-found
kubectl apply -f k8s/migrator-job.yaml


echo "[9/9] Starting port-forward from service api-service to localhost:$LOCAL_PORT"
kubectl port-forward svc/api-service "$LOCAL_PORT":8080
