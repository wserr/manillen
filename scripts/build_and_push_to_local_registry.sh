# Push frontend to local registry
docker build ../src/frontend/ -t localhost:32000/manillen-frontend:latest --build-arg BUILD_CMD=build-local-cluster
docker push localhost:32000/manillen-frontend:latest

# Push chat backend to local registry
docker build ../src/chat/ -t localhost:32000/manillen-chat:latest
docker push localhost:32000/manillen-chat:latest