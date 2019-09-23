# Rust Death Toy - In Progress

A rust implementation of [this simulation](https://github.com/jillagal/deathToy).
Original paper can be found [here](https://www.nature.com/articles/s41598-019-39636-x).

## Building
Run `cargo build` in root directory.

## Differences
Instead of coercing the off-lattice model into a lattice and checking Moore neighborhoods
for interaction this implementation uses a k-d tree. At each from a k-d tree is created
from the cells in the simulation. Though not yet implemented nearby cells can be found
with a range query into this k-d tree (see TODO).

## TODO
- Cell-Cell interactions (bumping into each other, checking for division angles)
- Different tradeoffs
- Cell death
