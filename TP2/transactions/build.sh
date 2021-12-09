#!/bin/bash

function build-release {
  sh -c "cd $1 && cargo build --release"
}

build-release 'alglobo'
build-release 'service'
