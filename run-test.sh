#!/bin/bash
cd "$(dirname $0)"
if [ -z "$1" ]
then
    bash ./roundup.sh tests/*.sh
else
    bash ./roundup.sh "$@"
fi
