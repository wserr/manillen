FROM jboss/keycloak:latest

COPY ./init/realm-export.json /tmp/