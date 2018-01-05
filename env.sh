#!/bin/sh

if [ "$1" = "" ]; then
    env="local"
elif [ "$1" = "local" ]; then
    env="local"
elif [ "$1" = "prod" ]; then
    env="prod"
else
    env="local"
fi

echo "ENV:$env"

cp "env/$env.conf" "nginx/sites/$env.conf"
cp "env/$env.docker-compose.yml" "docker-compose.yml"
cp "env/$env.env" ".env"

echo "Finished SetEnv"
