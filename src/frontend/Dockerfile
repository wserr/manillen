FROM node:18-alpine3.15 as BASE

ARG BUILD_CMD="build"

# Create app directory
WORKDIR /usr/src/app
COPY package*.json ./

RUN npm install

# Bundle app source 
COPY . .

WORKDIR /usr/src/app/

RUN npm run ${BUILD_CMD}

FROM nginx:latest as PUBLISH
WORKDIR /var/www/app

COPY --from=BASE /usr/src/app/build/ /usr/share/nginx/html

EXPOSE 80