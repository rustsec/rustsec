//! Dependency trees computed from `Cargo.lock` files.
//!
//! Uses the `petgraph` crate for modeling the dependency structure.

// Includes code from `cargo-tree`, Copyright (c) 2015-2016 Steven Fackler
// Licensed under the same terms as `cargo-audit` (i.e. Apache 2.0 + MIT)

use super::{
    graph::{EdgeDirection, Graph, NodeIndex, Nodes},
    Dependency,
};
use crate::{error::Error, lockfile::Lockfile, package::Package, Map};
use std::{collections::BTreeSet as Set, fmt};

/// Dependency tree computed from a `Cargo.lock` file
#[derive(Clone, Debug)]
pub struct Tree {
    /// Dependency graph for a particular package
    graph: Graph,

    /// Package data associated with nodes in the graph
    nodes: Nodes,

    /// Root node of the dependency graph
    root_index: NodeIndex,
}

impl Tree {
    /// Construct a new dependency tree for the given [`Lockfile`].
    pub fn new(lockfile: &Lockfile) -> Result<Self, Error> {
        let mut graph = Graph::new();
        let mut nodes = Map::new();

        let root_package = lockfile.root_package();
        let root_node = graph.add_node(root_package.clone());

        // TODO(tarcieri): index nodes by (name, version) tuples
        nodes.insert(Dependency::from(root_package.clone()), root_node);

        let mut dep_graph = Self {
            graph,
            nodes,
            root_index: root_node,
        };

        dep_graph.add_dependencies(lockfile, root_package, root_node);
        Ok(dep_graph)
    }

    /// Display the dependency graph for the given [`NodeIndex`] using the
    /// default set of [`Symbols`].
    pub fn display(
        &self,
        f: &mut fmt::Formatter<'_>,
        node_index: NodeIndex,
        direction: EdgeDirection,
    ) -> fmt::Result {
        self.display_with_symbols(f, node_index, direction, &Symbols::default())
    }

    /// Display the dependency graph for the given [`NodeIndex`] using the
    /// provided set of [`Symbols`].
    pub fn display_with_symbols(
        &self,
        f: &mut fmt::Formatter<'_>,
        node_index: NodeIndex,
        direction: EdgeDirection,
        symbols: &Symbols,
    ) -> fmt::Result {
        Presenter::new(&self.graph, symbols).print_node(f, node_index, direction)
    }

    /// Get the `petgraph` dependency graph.
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Get the nodes of the `petgraph` dependency graph.
    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    /// Get the root [`NodeIndex`] in the dependency graph.
    pub fn root_index(&self) -> NodeIndex {
        self.root_index
    }

    /// Get the root [`Package`] in the dependency graph
    pub fn root_package(&self) -> &Package {
        &self.graph[self.root_index]
    }

    /// Add the dependencies of a given package to the given node
    fn add_dependencies(&mut self, lockfile: &Lockfile, package: &Package, parent: NodeIndex) {
        for dependent_package in lockfile.dependent_packages(package) {
            let dependency = Dependency::from(dependent_package.clone());
            let node_index = self.graph.add_node(dependent_package.clone());
            self.nodes.insert(dependency.clone(), node_index);
            self.graph.add_edge(parent, node_index, dependency);
            self.add_dependencies(lockfile, dependent_package, node_index);
        }
    }
}

/// Symbols to use when printing the dependency tree
pub struct Symbols {
    down: &'static str,
    tee: &'static str,
    ell: &'static str,
    right: &'static str,
}

impl Default for Symbols {
    fn default() -> Symbols {
        Self {
            down: "│",
            tee: "├",
            ell: "└",
            right: "─",
        }
    }
}

/// Dependency tree presenter
struct Presenter<'g, 's> {
    /// Dependency graph being displayed
    graph: &'g Graph,

    /// Symbols to use to display graph
    symbols: &'s Symbols,

    /// Are there continuing levels?
    levels_continue: Vec<bool>,

    /// Dependencies we've already visited
    visited: Set<NodeIndex>,
}

impl<'g, 's> Presenter<'g, 's> {
    /// Create a new dependency tree `Presenter`.
    fn new(graph: &'g Graph, symbols: &'s Symbols) -> Self {
        Self {
            graph,
            symbols,
            levels_continue: vec![],
            visited: Set::new(),
        }
    }

    /// Print a node in the dependency tree.
    fn print_node(
        &mut self,
        f: &mut fmt::Formatter<'_>,
        node_index: NodeIndex,
        direction: EdgeDirection,
    ) -> fmt::Result {
        let package = &self.graph[node_index];
        let new = self.visited.insert(node_index);

        if let Some((&last_continues, rest)) = self.levels_continue.split_last() {
            for &continues in rest {
                let c = if continues { self.symbols.down } else { " " };
                write!(f, "{}   ", c)?;
            }

            let c = if last_continues {
                self.symbols.tee
            } else {
                self.symbols.ell
            };

            write!(f, "{0}{1}{1} ", c, self.symbols.right)?;
        }

        writeln!(f, "{} {}", &package.name, &package.version)?;

        if !new {
            return Ok(());
        }

        use petgraph::visit::EdgeRef;
        let dependencies = self
            .graph
            .edges_directed(node_index, direction)
            .map(|edge| edge.source())
            .collect::<Vec<_>>();

        for (i, dependency) in dependencies.iter().enumerate() {
            self.levels_continue.push(i < (dependencies.len() - 1));
            self.print_node(f, *dependency, direction)?;
            self.levels_continue.pop();
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Load the `rustsec` crate's `Cargo.lock`
    fn load_lockfile() -> Lockfile {
        Lockfile::load("Cargo.lock").unwrap()
    }

    #[test]
    fn compute_tree() {
        // TODO(tarcieri): test dependency tree is computed correctly
        Tree::new(&load_lockfile()).unwrap();
    }
}
