#!/bin/bash

cmd="$1"
shift
[[ $# -le 0 ]] && exit

bash -c "$cmd $*"
