//! Dependency graphs computed from `Cargo.lock` files.
//!
//! Uses the `petgraph` crate for modeling the dependency structure.

pub use petgraph;

use crate::{
    error::{Error, ErrorKind},
    lockfile::Lockfile,
    package::{self, Package},
};
use petgraph::graph::NodeIndex;
use std::collections::BTreeMap as Map;

/// Dependency graph (modeled using `petgraph`)
pub type Graph = petgraph::graph::Graph<Package, Edge>;

/// Nodes in the dependency graph
pub type Nodes = Map<package::Release, NodeIndex>;

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
    pub fn new(lockfile: &Lockfile) -> Result<Self, Error> {
        let mut graph = Graph::new();
        let mut nodes = Map::new();

        let root_package = find_root_package(lockfile)?;
        let root_node = graph.add_node(root_package.clone());

        // TODO(tarcieri): index nodes by (name, version) tuples
        nodes.insert(root_package.release(), root_node);

        let mut dep_graph = Self {
            graph,
            nodes,
            root_node,
        };

        dep_graph.add_dependencies(lockfile, root_package, root_node);
        Ok(dep_graph)
    }

    /// Get the dependency graph
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Get the nodes of the dependency graph
    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    /// Get the root package in the dependency graph
    pub fn root_package(&self) -> &Package {
        &self.graph[self.root_node]
    }

    /// Add the dependencies of a given package to the given node
    fn add_dependencies(&mut self, lockfile: &Lockfile, package: &Package, parent: NodeIndex) {
        for dependency in lockfile.dependencies(package) {
            let node = self.graph.add_node(dependency.clone());
            self.nodes.insert(dependency.release(), node);
            self.graph.add_edge(parent, node, Edge);
            self.add_dependencies(lockfile, dependency, node);
        }
    }
}

/// Find the root package in the given lockfile
fn find_root_package(lockfile: &Lockfile) -> Result<&Package, Error> {
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
        .ok_or_else(|| format_err!(ErrorKind::Parse, "couldn't find root package"))?
        .0;

    Ok(lockfile
        .packages
        .iter()
        .find(|package| &package.name == root_package_name)
        .unwrap())
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
        Lockfile::load("Cargo.lock").unwrap()
    }

    #[test]
    fn root_package() {
        let lockfile = load_lockfile();
        assert_eq!(
            find_root_package(&lockfile).unwrap().name.as_str(),
            "cargo-lock"
        );
    }

    #[test]
    fn computed_graph() {
        let lockfile = load_lockfile();
        let dependencies = DependencyGraph::new(&lockfile).unwrap();
        let root_package = dependencies.root_package();
        assert_eq!(root_package.name.as_str(), "cargo-lock");

        // TODO(tarcieri): test dependency graph construction
    }
}
