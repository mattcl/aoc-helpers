use std::{collections::BinaryHeap, fmt::Debug};

use num::{Num, Bounded};

use super::{Grid, Location};


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

pub trait CostCache<T, G>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    fn cache_get(&self, id: &T) -> G;
    fn cache_set(&mut self, id: &T, val: G);
}

pub trait Dijkstra<T, G>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    fn shortest_path_cost<Cache, EdgeFn>(&self, start: T, goal: T, cost_cache: &mut Cache, edges_fn: EdgeFn) -> Option<G>
    where
        Cache: CostCache<T, G>,
        EdgeFn: Fn(&T) -> Vec<DEdge<T, G>>
    {
        let mut heap = BinaryHeap::new();

        let start = DNode {id: start, cost: G::zero()};
        cost_cache.cache_set(&start.id, G::zero());
        heap.push(start);

        while let Some(DNode {id, cost}) = heap.pop() {
            if id == goal {
                return Some(cost)
            }

            if cost > cost_cache.cache_get(&id) {
                continue;
            }

            for edge in edges_fn(&id) {
                let next = DNode {id: edge.id, cost: cost + edge.cost};

                if next.cost < cost_cache.cache_get(&next.id) {
                    cost_cache.cache_set(&next.id, next.cost);
                    heap.push(next);
                }
            }
        }

        None
    }
}

impl<G, T> Dijkstra<Location, G> for Grid<T>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy
{ }
