//! Graph-based topology representation

use petgraph::graph::{DiGraph, NodeIndex};

/// Unique identifier for a bus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BusId(pub usize);

/// Unique identifier for a branch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BranchId(pub usize);

/// Node data in the topology graph
#[derive(Debug, Clone)]
pub struct TopologyNode {
    pub bus_id: BusId,
}

/// Edge data in the topology graph
#[derive(Debug, Clone)]
pub struct TopologyEdge {
    pub branch_id: BranchId,
}

/// Graph-based network topology
#[derive(Debug, Clone)]
pub struct Topology {
    graph: DiGraph<TopologyNode, TopologyEdge>,
}

impl Topology {
    /// Create a new empty topology
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
        }
    }

    /// Add a bus to the topology
    pub fn add_bus(&mut self, bus_id: BusId) -> NodeIndex {
        self.graph.add_node(TopologyNode { bus_id })
    }

    /// Add a branch between two buses
    pub fn add_branch(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        branch_id: BranchId,
    ) {
        self.graph.add_edge(from, to, TopologyEdge { branch_id });
        // Add reverse edge for undirected graph behavior
        self.graph.add_edge(to, from, TopologyEdge { branch_id });
    }

    /// Number of buses (nodes)
    pub fn bus_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Number of branches (edges / 2 for undirected)
    pub fn branch_count(&self) -> usize {
        self.graph.edge_count() / 2
    }

    /// Get reference to internal graph
    pub fn graph(&self) -> &DiGraph<TopologyNode, TopologyEdge> {
        &self.graph
    }
}

impl Default for Topology {
    fn default() -> Self {
        Self::new()
    }
}
