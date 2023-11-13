use std::borrow::Borrow;

use slotmap::HopSlotMap;

use crate::{
    graphs::{keys::NodeIdx, Graph},
    utils::wrapped_indices_iterator::WrappedIndicesIterator,
};

/// An iterator which converts `(&)NodeIdx` to a `&'g N` of the graph
pub struct NodesByIdx<'g, N: 'g, B: Borrow<NodeIdx>, I: Iterator<Item = B>> {
    nodes: &'g HopSlotMap<NodeIdx, N>,
    inner: I,
}

impl<'g, N: 'g, B: Borrow<NodeIdx>, I: Iterator<Item = B>> NodesByIdx<'g, N, B, I> {
    /// Creates a new `NodesByIdx` iterator over a graph with the provided `inner` iterator
    pub fn from_graph<E>(inner: I, graph: &'g impl Graph<N, E>) -> Self {
        Self {
            nodes: unsafe { graph.nodes_raw() },
            inner,
        }
    }

    /// Creates a new `NodesByIdx` iterator over a graph with the provided `inner` iterator
    pub fn new(inner: I, nodes: &'g HopSlotMap<NodeIdx, N>) -> Self {
        Self { nodes, inner }
    }
}

impl<'g, N: 'g, I: Iterator<Item = &'g NodeIdx>> WrappedIndicesIterator<NodeIdx>
    for NodesByIdx<'g, N, &'g NodeIdx, I>
{
    type IndicesIter = std::iter::Cloned<I>;

    #[inline]
    fn into_indices(self) -> Self::IndicesIter {
        self.inner.cloned()
    }
}

impl<'g, N: 'g, I: Iterator<Item = NodeIdx>> WrappedIndicesIterator<NodeIdx>
    for NodesByIdx<'g, N, NodeIdx, I>
{
    type IndicesIter = I;

    #[inline]
    fn into_indices(self) -> Self::IndicesIter {
        self.inner
    }
}

impl<'g, N: 'g, B: Borrow<NodeIdx>, I: Iterator<Item = B>> Iterator for NodesByIdx<'g, N, B, I> {
    type Item = &'g N;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.inner.next() {
            self.nodes.get(*index.borrow())
        } else {
            None
        }
    }
}
