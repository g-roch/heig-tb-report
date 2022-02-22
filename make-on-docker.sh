#!/bin/bash

DOCKERIMAGE=heig-tb-docker-image

docker build -t $DOCKERIMAGE .github/action/

docker run --rm -v $(pwd):/mnt -w /mnt -u $(id -u):$(id -g) $DOCKERIMAGE make $@
