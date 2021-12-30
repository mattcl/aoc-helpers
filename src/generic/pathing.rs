use std::{collections::BinaryHeap, fmt::Debug};

use num::{Bounded, Num};

use super::{Grid, Location};

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DNode<T, G>
where
    G: Num,
{
    id: T,
    cost: G,
}

impl<T, G> Ord for DNode<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T, G> PartialOrd for DNode<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    id: T,
    cost: G,
}

impl<T, G> DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    pub fn new(id: T, cost: G) -> Self {
        Self { id, cost }
    }
}

impl<T, G> From<(T, G)> for DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn from(v: (T, G)) -> Self {
        Self::new(v.0, v.1)
    }
}

pub trait CostCache<T> {
    type Cost: Num + Bounded + Ord + PartialOrd + Clone + Copy;

    fn cache_get(&self, id: &T) -> Self::Cost;
    fn cache_set(&mut self, id: &T, val: Self::Cost);
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DefaultLocationCache<G>
where
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    elements: Vec<G>,
    row_count: usize,
}

impl<G> DefaultLocationCache<G>
where
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    pub fn new(size: usize, row_count: usize) -> Self {
        Self {
            elements: vec![G::max_value(); size],
            row_count,
        }
    }
}

impl<G> CostCache<Location> for DefaultLocationCache<G>
where
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    type Cost = G;

    fn cache_get(&self, id: &Location) -> Self::Cost {
        self.elements[id.as_rm_index(self.row_count)]
    }

    fn cache_set(&mut self, id: &Location, val: Self::Cost) {
        self.elements[id.as_rm_index(self.row_count)] = val;
    }
}

pub fn dijkstra_cost<T, G, Cache, EdgeFn>(
    start: T,
    goal: T,
    cost_cache: &mut Cache,
    edges_fn: EdgeFn,
) -> Option<G>
where
    T: Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
    Cache: CostCache<T, Cost = G>,
    EdgeFn: Fn(&T) -> Vec<DEdge<T, G>>,
{
    let mut heap = BinaryHeap::new();

    let start = DNode {
        id: start,
        cost: G::zero(),
    };
    cost_cache.cache_set(&start.id, G::zero());
    heap.push(start);

    while let Some(DNode { id, cost }) = heap.pop() {
        if id == goal {
            return Some(cost);
        }

        if cost > cost_cache.cache_get(&id) {
            continue;
        }

        for edge in edges_fn(&id) {
            let next = DNode {
                id: edge.id,
                cost: cost + edge.cost,
            };

            if next.cost < cost_cache.cache_get(&next.id) {
                cost_cache.cache_set(&next.id, next.cost);
                heap.push(next);
            }
        }
    }

    None
}
