use std::{collections::BinaryHeap, fmt::Debug};

use num::{Bounded, Num};

use super::Location;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DNode<T, G>
where
    G: Num,
{
    pub id: T,
    pub cost: G,
}

impl<T, G> Ord for DNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl<T, G> PartialOrd for DNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DPNode<T, G>
where
    G: Num,
{
    pub id: T,
    pub cost: G,
    pub path: Vec<T>,
}

impl<T, G> Ord for DPNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl<T, G> PartialOrd for DPNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct DASTNode<T, G>
where
    G: Num,
{
    pub id: T,
    pub cost: G,
    pub path: G,
}

impl<T, G> Ord for DASTNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.id.cmp(&self.id))
    }
}

impl<T, G> PartialOrd for DASTNode<T, G>
where
    T: Ord + PartialOrd + Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DEdge<T, G>
where
    T: Eq + PartialEq,
    G: Num + Ord + PartialOrd,
{
    pub id: T,
    pub cost: G,
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
    col_count: usize,
}

impl<G> DefaultLocationCache<G>
where
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    pub fn new(size: usize, col_count: usize) -> Self {
        Self {
            elements: vec![G::max_value(); size],
            col_count,
        }
    }
}

impl<G> CostCache<Location> for DefaultLocationCache<G>
where
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
{
    type Cost = G;

    fn cache_get(&self, id: &Location) -> Self::Cost {
        self.elements[id.as_rm_index(self.col_count)]
    }

    fn cache_set(&mut self, id: &Location, val: Self::Cost) {
        self.elements[id.as_rm_index(self.col_count)] = val;
    }
}

pub fn dijkstra_cost<T, G, Cache, EdgeFn>(
    start: T,
    goal: T,
    cost_cache: &mut Cache,
    edges_fn: EdgeFn,
) -> Option<G>
where
    T: Ord + PartialOrd + Eq + PartialEq + Debug + Clone,
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

pub fn dijkstra_path<T, G, Cache, EdgeFn>(
    start: T,
    goal: T,
    cost_cache: &mut Cache,
    edges_fn: EdgeFn,
) -> Option<Vec<T>>
where
    T: Ord + PartialOrd + Eq + PartialEq + Debug + Clone,
    G: Num + Bounded + Ord + PartialOrd + Clone + Copy,
    Cache: CostCache<T, Cost = G>,
    EdgeFn: Fn(&T) -> Vec<DEdge<T, G>>,
{
    let mut heap = BinaryHeap::new();

    let start = DPNode {
        // ideally, we'd copy here, but that might impose too strict of a
        // restriction on T
        id: start.clone(),
        cost: G::zero(),
        path: vec![start],
    };
    cost_cache.cache_set(&start.id, G::zero());
    heap.push(start);

    while let Some(DPNode { id, cost, path }) = heap.pop() {
        if id == goal {
            return Some(path);
        }

        if cost > cost_cache.cache_get(&id) {
            continue;
        }

        for edge in edges_fn(&id) {
            let mut new_path = path.clone();
            // again, would prefer copy but that would require imposing copy on
            // T, which is not something I want to do
            new_path.push(edge.id.clone());

            let next = DPNode {
                id: edge.id,
                cost: cost + edge.cost,
                path: new_path,
            };

            if next.cost < cost_cache.cache_get(&next.id) {
                cost_cache.cache_set(&next.id, next.cost);
                heap.push(next);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_ordering() {
        let mut nodes = vec![
            DNode {
                id: Location::new(0, 0),
                cost: 5_usize,
            },
            DNode {
                id: Location::new(0, 2),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(0, 0),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(0, 1),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(1, 1),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(1, 1),
                cost: 7_usize,
            },
        ];

        let expected = vec![
            DNode {
                id: Location::new(1, 1),
                cost: 7_usize,
            },
            DNode {
                id: Location::new(0, 0),
                cost: 5_usize,
            },
            DNode {
                id: Location::new(1, 1),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(0, 2),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(0, 1),
                cost: 4_usize,
            },
            DNode {
                id: Location::new(0, 0),
                cost: 4_usize,
            },
        ];
        nodes.sort();

        assert_eq!(nodes, expected);
    }
}
