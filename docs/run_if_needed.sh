#!/bin/bash
set -euo pipefail
IFS=$'\n\t'

cmd=$1
[[ $# -le 1 ]] && exit

bash -c "$cmd $2"
