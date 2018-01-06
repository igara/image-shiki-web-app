#!/bin/sh

if [ "$1" = "" ]; then
    env="local"
elif [ "$1" = "local" ]; then
    env="local"
elif [ "$1" = "production" ]; then
    env="production"
else
    env="local"
fi

echo "ENV:$env"

cp "env/$env.docker-compose.yml" "docker-compose.yml"
cp "env/$env.env" ".env"

echo "Finished SetEnv"
