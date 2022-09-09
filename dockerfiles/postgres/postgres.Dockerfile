FROM postgres:latest

# Create Keycloak database
COPY ./init/init-keycloak-db.sh /docker-entrypoint-initdb.d/