# Route-planning

This project implements two heuristic search algorithms for route planning on a weighted undirected graph:

- A* search
- Greedy Best-First Search

Both programs read a road network from standard input and print the shortest known route cost to the goal, or `NO PATH` when no route exists.

## Project Structure

- `src/bin/astar.rs` - A* implementation
- `src/bin/greedy.rs` - Greedy Best-First Search implementation
- `tests/astar_test.rs` - Basic integration tests for the A* binary
- `benchmark.md` - Notes comparing the two algorithms

## Input Format

The programs expect input in the following format:

```text
n m
u1 v1 w1
u2 v2 w2
...
start goal
```

Where:
- `n` is the number of cities
- `m` is the number of roads
- each road entry is `u v w` for an undirected edge with weight `w`
- `start` and `goal` are the source and destination cities

## Example

```text
5 6
1 2 4
1 3 2
2 4 5
3 4 1
4 5 3
3 5 8
1 5
```

## Build and Run

Build the project with:

```bash
cargo build
```

Run the A* binary:

```bash
cargo run --bin astar
```

Run the Greedy binary:

```bash
cargo run --bin greedy
```

## Testing

Run the test suite with:

```bash
cargo test
```

## Notes

The heuristic used in both implementations is based on the absolute difference between node numbers. This is simple and admissible for the test cases in this project.
