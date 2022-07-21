## Script that adds secrets for service settings
microk8s kubectl create secret generic keycloak-secret \
  --from-literal=databasehost='postgres' \
  --from-literal=databaseport='5432' \
  --from-literal=adminuser='admin' \
  --from-literal=adminpassword='admin' \
  --from-literal=databaseuser='postgres' \
  --from-literal=databasepassword='postgres' -n manillen

microk8s kubectl create secret generic postgres-secret \
  --from-literal=user='postgres' \
  --from-literal=password='postgres' -n manillen