#!/bin/zsh

docker stop touristdb
docker rm touristdb
docker run --name touristdb -p 5432:5432 -d -v touristdb:/var/lib/postgresql/data touristdb


cargo r
