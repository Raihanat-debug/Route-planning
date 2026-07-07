use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: i32,
}

fn heuristic(node: usize, goal: usize) -> i32 {
    // Same heuristic used in A*
    (goal as i32 - node as i32).abs()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    // Number of cities and roads
    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut graph = vec![Vec::<Edge>::new(); n + 1];

    // Read the roads (undirected graph)
    for _ in 0..m {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();

        graph[u].push(Edge { to: v, cost: w });
        graph[v].push(Edge { to: u, cost: w });
    }

    let start: usize = iter.next().unwrap().parse().unwrap();
    let goal: usize = iter.next().unwrap().parse().unwrap();

    let mut visited = vec![false; n + 1];

    // (heuristic, total_cost, node)
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((heuristic(start, goal), 0, start)));

    while let Some(Reverse((_, cost, node))) = pq.pop() {
        if visited[node] {
            continue;
        }

        visited[node] = true;

        if node == goal {
            println!("{}", cost);
            return;
        }

        for edge in &graph[node] {
            if !visited[edge.to] {
                let new_cost = cost + edge.cost;

                pq.push(Reverse((
                    heuristic(edge.to, goal),
                    new_cost,
                    edge.to,
                )));
            }
        }
    }

    println!("NO PATH");
}