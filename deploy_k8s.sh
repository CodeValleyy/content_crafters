#!/bin/sh

project_id=pa2024-421814

read -p "Souhaitez-vous déployer content_crafters ? (y/n) " deploy_content_crafters
read -p "Souhaitez-vous déployer mongo-db ? (y/n) " deploy_mongo

if [ "$deploy_content_crafters" = "y" ] || [ "$deploy_content_crafters" = "Y" ]; then
  docker build -t content_crafters_api/content-crafters:latest .
  docker tag content_crafters_api/content-crafters:latest gcr.io/$project_id/content-crafters:latest
  docker push gcr.io/$project_id/content-crafters:latest
fi

kubectl apply -f secret.yaml

if [ "$deploy_mongo" = "y" ] || [ "$deploy_mongo" = "Y" ]; then
  kubectl apply -f mongodb-deployment.yaml
fi

kubectl apply -f deployment.yaml
kubectl apply -f service.yaml

if [ "$deploy_content_crafters" = "y" ] || [ "$deploy_content_crafters" = "Y" ]; then
  kubectl set image deployment/content-crafters-deployment content-crafters=gcr.io/$project_id/content-crafters:latest
  kubectl rollout restart deployment/content-crafters-deployment
fi
