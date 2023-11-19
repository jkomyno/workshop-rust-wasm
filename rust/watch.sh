#!/bin/bash

GLOB=${1:-""}

cargo watch -s "./build.sh ${GLOB}"
