docker build . -f docker/Dockerfile -t littlebunch/graphql-rs
docker push littlebunch/graphql-rs
docker run --rm -it -p 8000:8000 --env-file=./docker.env littlebunch/graphql-rs
