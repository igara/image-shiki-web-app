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
cp "env/$env.env" ".env"
cp "env/$env.www.sh" "rust/www/www.sh"

echo "Finished SetEnv"
