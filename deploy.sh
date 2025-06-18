#!/bin/bash
set -e

IMAGE_NAME="ethans333/shop-app_api:latest"
KUBERNETES_DIR="./k8s"

echo "[1/5] Pulling image: $IMAGE_NAME"
docker pull $IMAGE_NAME

echo "[2/5] Loading image into kind cluster"
kind load docker-image $IMAGE_NAME

echo "[3/5] Applying for Kubernetes"
for entry in "$KUBERNETES_DIR"/*; 
    do
        if [ -d "$entry" ]; then
            kubectl apply -f "$entry/deployment.yaml"
            kubectl apply -f "$entry/service.yaml"
        fi
    done

echo "[5/5] Port forwarding from pod (Ctrl+C to cancel)"
kubectl port-forward svc/api-service 8080:8080
