#!/bin/bash

pwd=$PWD

if [ "$#" -ne 3 ]; then
    echo "Usage: bash $0 <level> <level> <task number>"
    exit 1
fi

cd $pwd/src/$1/$2/$3/ && cargo build && cargo run && cd $pwd
