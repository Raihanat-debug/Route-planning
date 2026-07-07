use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    to: usize,
    cost: i32,
}

fn heuristic(node: usize, goal: usize) -> i32 {
    // Simple admissible heuristic.
    // Since we have no coordinates, we use the absolute
    // difference between node numbers.
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

    // Roads are undirected
    for _ in 0..m {
        let u: usize = iter.next().unwrap().parse().unwrap();
        let v: usize = iter.next().unwrap().parse().unwrap();
        let w: i32 = iter.next().unwrap().parse().unwrap();

        graph[u].push(Edge { to: v, cost: w });
        graph[v].push(Edge { to: u, cost: w });
    }

    let start: usize = iter.next().unwrap().parse().unwrap();
    let goal: usize = iter.next().unwrap().parse().unwrap();

    let mut dist = vec![i32::MAX; n + 1];

    let mut pq = BinaryHeap::new();

    dist[start] = 0;

    pq.push(Reverse((heuristic(start, goal), 0, start)));

    while let Some(Reverse((_, cost, node))) = pq.pop() {
        if node == goal {
            println!("{}", cost);
            return;
        }

        if cost > dist[node] {
            continue;
        }

        for edge in &graph[node] {
            let new_cost = cost + edge.cost;

            if new_cost < dist[edge.to] {
                dist[edge.to] = new_cost;

                let priority = new_cost + heuristic(edge.to, goal);

                pq.push(Reverse((priority, new_cost, edge.to)));
            }
        }
    }

    println!("NO PATH");
}