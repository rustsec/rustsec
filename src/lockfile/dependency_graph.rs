//! Dependency graphs computed from `Cargo.lock` files.
//!
//! Uses the `petgraph` crate for modeling the dependency structure.

pub use petgraph;

use crate::{
    lockfile::Lockfile,
    package::{self, Package},
};
use petgraph::graph::NodeIndex;
use std::collections::BTreeMap as Map;

/// Dependency graph (modeled using `petgraph`)
pub type Graph = petgraph::graph::Graph<Package, Edge>;

/// Nodes in the dependency graph
pub type Nodes = Map<package::Name, NodeIndex>;

/// Dependency graph computed from a `Cargo.lock` file
#[derive(Clone, Debug)]
pub struct DependencyGraph {
    /// Dependency graph for a particular package
    graph: Graph,

    /// Package data associated with nodes in the graph
    nodes: Nodes,

    /// Root node of the dependency graph
    root_node: NodeIndex,
}

impl DependencyGraph {
    /// Construct a new dependency graph for the given [`Lockfile`]
    pub fn new(lockfile: &Lockfile) -> Self {
        let mut graph = Graph::new();
        let mut nodes = Map::new();

        let root_package = find_root_package(lockfile);
        let root_node = graph.add_node(root_package.clone());

        // TODO(tarcieri): index nodes by (name, version) tuples
        nodes.insert(root_package.name.clone(), root_node);

        let mut dep_graph = Self {
            graph,
            nodes,
            root_node,
        };

        dep_graph.add_dependencies(lockfile, root_package, root_node);
        dep_graph
    }

    /// Get the dependency graph
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Get the nodes of the dependency graph
    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    /// Get the root node index in the dependency graph
    pub fn root_package(&self) -> &Package {
        &self.graph[self.root_node]
    }

    /// Add the dependencies of a given package to the given node
    fn add_dependencies(&mut self, lockfile: &Lockfile, package: &Package, parent: NodeIndex) {
        for dependency in lockfile.dependencies(package) {
            let node = self.graph.add_node(dependency.clone());
            self.nodes.insert(dependency.name.clone(), node);
            self.graph.add_edge(parent, node, Edge);
            self.add_dependencies(lockfile, dependency, node);
        }
    }
}

/// Find the root package in the given lockfile
fn find_root_package(lockfile: &Lockfile) -> &Package {
    let mut dependency_counts = Map::new();

    for package in &lockfile.packages {
        dependency_counts.entry(&package.name).or_insert(0);

        for dependency in &package.dependencies {
            *dependency_counts.entry(&dependency.name).or_insert(0) += 1;
        }
    }

    let root_package_name = *dependency_counts
        .iter()
        .find(|(_, count)| **count == 0)
        .expect("couldn't find root package!")
        .0;

    lockfile
        .packages
        .iter()
        .find(|package| &package.name == root_package_name)
        .unwrap()
}

/// Graph edges. These are presently just a placeholder.
// TODO(tarcieri): use these for e.g. VersionReqs?
#[derive(Clone, Debug)]
pub struct Edge;

#[cfg(test)]
mod tests {
    use super::*;

    /// Load the `rustsec` crate's `Cargo.lock`
    fn load_lockfile() -> Lockfile {
        Lockfile::load_file("Cargo.lock").unwrap()
    }

    #[test]
    fn root_package() {
        let lockfile = load_lockfile();
        assert_eq!(find_root_package(&lockfile).name.as_str(), "rustsec");
    }

    #[test]
    fn computed_graph() {
        let lockfile = load_lockfile();
        let dependencies = DependencyGraph::new(&lockfile);
        let root_package = dependencies.find_by_index(dependencies.root_index());
        assert_eq!(root_package.name.as_str(), "rustsec");

        // TODO(tarcieri): test dependency graph construction
    }
}
