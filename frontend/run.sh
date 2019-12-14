#!/bin/sh
docker build -t vuejs .
docker run --name www-frontend -p 80:8080 -d vuejs
