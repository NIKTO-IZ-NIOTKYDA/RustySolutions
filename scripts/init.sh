#!/bin/bash

if [ "$#" -ne 3 ]; then
    echo "Usage: bash $0 <level> <level> <task number>"
    exit 1
fi

echo Running: cargo init --name solution_"$1"_"$2"_"$3" --vcs=none "$PWD/src/$1/$2/$3"
cargo init --name solution_"$1"_"$2"_"$3" --vcs=none "$PWD/src/$1/$2/$3" > /dev/null 2>&1
status_code=$?

echo "/*
    Task:          $3
    Level:         $1.$2
    Author:        https://github.com/NIKTO-IZ-NIOTKYDA
    Task URL:      https://code.mu/en/rust/tasker/stager/$1/$2/
    Repository:    https://github.com/NIKTO-IZ-NIOTKYDA/RustySolutions
*/

fn main() {
    println!(\"Hello, World!\");
}" > $PWD/src/$1/$2/$3/src/main.rs

if [ $status_code -ne 0 ]; then
    echo Error: The cargo init --name solution_"$1"_"$2"_"$3" --vcs=none "$PWD/src/$1/$2/$3" team ended with an error" (error code: $status_code)"
    exit 1
fi

echo Project successfully initialized: solution_"$1"_"$2"_"$3"
