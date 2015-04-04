# Octree

An Octree implementation in Rust. Mainly for learning a new language!

## Synopsis

This library creates a octree of a certain size based on a given value function.
The algorithm checks if there is a significant difference between the values inside the cube.
The function it uses to determine the values is the previously named value function.
In this example it's a check if it's inside a sphere. The result is then raycast and outputted to a png.


## Requirements

rust & cargo and optional gnuplot

## How to use

fiddle with the code as you wish. To display the result I'm doing:

1. cargo run
2. ./target/debug/octree > out.txt
3. gnuplot analyze.plt

