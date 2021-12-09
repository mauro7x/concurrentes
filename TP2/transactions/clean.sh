#!/bin/bash

function clean {
  sh -c "cd $1 && cargo clean"
}

clean 'alglobo'
clean 'service'
