#!/bin/sh

if [ $# -eq 0 ] || [ $# -gt 2 ]; then
    echo "Usage: ./make <day>
       ./make prep <day>"
    exit 1
fi

if [ $1 = "4" ]; then
    cd day4 && cargo run
    cd ..
    exit
fi

if [ $1 = "prep" ]; then
    if [ -e $2.rs ]; then
        echo "$2.rs already exists."
        exit 1
    fi

    echo "const INPUT: &str = include_str!(\"$2.txt\");

fn main() { one(); two(); }

fn one() {
    todo!()
}

fn two() {
    todo!()
}" > $2.rs \
    && echo "Created $2.rs"
else
    rustc $1.rs -o $1.out && ./$1
fi
