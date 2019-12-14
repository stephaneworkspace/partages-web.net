#!/bin/sh
docker stop www-frontend
docker rm www-frontend
docker build -t vuejs .
docker run --name www-frontend -p 8080:80 -d vuejs
