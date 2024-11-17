#!/bin/sh

banner="    _          ____   ____   ___  _ ____  
   / \\   ___  / ___| |___ \\ / _ \\/ | ___| 
  / _ \\ / _ \\| |       __) | | | | |___ \\ 
 / ___ \\ (_) | |___   / __/| |_| | |___) |
/_/   \\_\\___/ \\____| |_____|\\___/|_|____/ 
"

if [ $# -eq 0 ] || [ $# -gt 2 ]; then
    echo "$banner"
    echo "Usage: ./make <day>
       ./make prep <day>
       ./make all"
    exit 1
fi

if [ $1 = "4" ]; then
    cd day4 && cargo run --quiet
    cd ..
    exit
fi

case $1 in
    "prep")
        if [ -e $2.rs ]; then
            echo "$2.rs already exists."
            exit 1
        fi

        echo "const INPUT: &str = include_str!(\"$2.txt\");

fn main() { one(); two(); }

fn one() {
    let result = 0;
    print!(\"$2.1: {result}\\t\\t\");
}

fn two() {
    let result = 0;
    println!(\"$2.2: {result}\");
}" > $2.rs && echo "Created $2.rs"
        ;;
    "all")
        echo "$banner"
        for day in {1..25}; do
            ./$0 $day
            if [ $? -ne 0 ]; then
                echo "=====================================================
One or more tasks not completed, starting from day $day.
====================================================="
                exit 1
            fi
        done

        echo "==============================================================
All tasks completed! Now make sure that the numbers are correct.
================================================================"
        ;;
    *)
        rustc $1.rs -o $1.out && ./$1.out
esac
