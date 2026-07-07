# Benchmark – Route Planning

## Objective

The objective is to implement a route planning system that finds a path between two cities in a weighted road network. Two heuristic search algorithms are compared:

1. A* Search
2. Greedy Best-First Search

Both implementations operate on the same weighted graph represented as an adjacency list. The comparison evaluates their theoretical performance, solution quality, and practical characteristics.

---

## Algorithm 1 – A* Search

A* (A-Star) is a heuristic search algorithm that combines the actual distance travelled with an estimate of the remaining distance to the goal.

The evaluation function is:

f(n) = g(n) + h(n)

where:

- g(n) is the cost from the start node to the current node.
- h(n) is the heuristic estimate from the current node to the goal.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O((V + E) log V) |
| Space | O(V) |

where:

- V = number of vertices (cities)
- E = number of edges (roads)

### Advantages

- Guarantees the shortest path when the heuristic is admissible.
- Usually explores fewer nodes than Dijkstra's algorithm.
- Widely used in GPS navigation, robotics, and video games.
- Efficient for weighted graphs.

### Disadvantages

- Requires a heuristic function.
- Can consume more memory because many nodes may be stored in the priority queue.

---

## Algorithm 2 – Greedy Best-First Search

Greedy Best-First Search also uses a heuristic function but ignores the distance already travelled.

The evaluation function is:

f(n) = h(n)

The algorithm always expands the node that appears closest to the destination according to the heuristic.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Time | O((V + E) log V) (worst case) |
| Space | O(V) |

### Advantages

- Often reaches the destination quickly.
- Usually explores fewer nodes than A*.
- Simple to implement.
- Suitable when a fast approximate solution is acceptable.

### Disadvantages

- Does not guarantee the shortest path.
- Can choose a longer route if the heuristic is misleading.

---

## Comparison

| Feature | A* | Greedy Best-First |
|---------|----|-------------------|
| Data Structure | Weighted Graph | Weighted Graph |
| Priority Function | g(n) + h(n) | h(n) |
| Shortest Path Guaranteed | Yes* | No |
| Uses Heuristic | Yes | Yes |
| Search Strategy | Cost + Heuristic | Heuristic Only |
| Typical Applications | GPS, Robotics, Games | AI Search, Route Suggestions |

*Provided that the heuristic is admissible.

---

## Conclusion

Both implementations correctly solve the Route Planning problem using heuristic search techniques.

A* combines the travelled distance with the heuristic estimate, allowing it to find an optimal route while reducing unnecessary exploration compared to uninformed search algorithms.

Greedy Best-First Search focuses solely on the heuristic estimate, making it faster in many situations but without guaranteeing the shortest route.

For applications where the shortest path is required, A* is generally the preferred algorithm. When speed is more important than optimality, Greedy Best-First Search provides a practical alternative.