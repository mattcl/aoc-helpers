use std::{collections::BinaryHeap, fmt::Debug};

use num::{Num, Bounded};

use super::{Grid, Location, prelude::GridLike};


#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DNode<T, G>
where
    G: Num
{
    id: T,
    cost: G,
}

impl<T, G> Ord for DNode<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T, G> PartialOrd for DNode<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd
{
    id: T,
    cost: G,
}

impl<T, G> DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd
{
    pub fn new(id: T, cost: G) -> Self {
        Self {id, cost}
    }
}

impl<T, G> From<(T, G)> for DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd
{
    fn from(v: (T, G)) -> Self {
        Self::new(v.0, v.1)
    }
}

pub trait Dijkstra<T, G>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    fn shortest_path_cost<F>(&self, start: T, goal: T, edges_fn: F) -> Option<G>
    where
        F: Fn(&T) -> Vec<DEdge<T, G>>;
}

impl<G, T> Dijkstra<Location, G> for Grid<T>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy
{
    fn shortest_path_cost<F>(&self, start: Location, goal: Location, edges_fn: F) -> Option<G>
    where
        F: Fn(&Location) -> Vec<DEdge<Location, G>>
    {
        let mut costs = vec![G::max_value(); self.size()];
        let mut heap = BinaryHeap::new();

        let start = DNode {id: start, cost: G::zero()};
        costs[start.id.as_rm_index(self.rows())] = G::zero();
        heap.push(start);

        while let Some(DNode {id, cost}) = heap.pop() {
            if id == goal {
                return Some(cost)
            }

            if cost > costs[id.as_rm_index(self.rows())] {
                continue;
            }

            for edge in edges_fn(&id).iter() {
                let next = DNode {id: edge.id, cost: cost + edge.cost};

                let idx = next.id.as_rm_index(self.rows());
                if next.cost < costs[idx] {
                    costs[idx] = next.cost;
                    heap.push(next);
                }
            }

        }

        None
    }
}
