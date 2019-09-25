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
use std::{collections::BTreeSet as Set, io};

/// Dependency tree computed from a `Cargo.lock` file
#[derive(Clone, Debug)]
pub struct Tree {
    /// Dependency graph for a particular package
    graph: Graph,

    /// Package data associated with nodes in the graph
    nodes: Nodes,
}

impl Tree {
    /// Construct a new dependency tree for the given [`Lockfile`].
    pub fn new(lockfile: &Lockfile) -> Result<Self, Error> {
        let mut tree = Self {
            graph: Graph::new(),
            nodes: Map::new(),
        };
        let mut indexed = Set::new();

        for package in &lockfile.packages {
            if !indexed.insert(package) {
                continue;
            }

            let node = tree.graph.add_node(package.clone());
            tree.nodes.insert(Dependency::from(package.clone()), node);
            tree.add_dependencies(lockfile, package, node, &mut indexed);
        }

        Ok(tree)
    }

    /// Render the dependency graph for the given [`NodeIndex`] using the
    /// default set of [`Symbols`].
    pub fn render(
        &self,
        w: &mut impl io::Write,
        node_index: NodeIndex,
        direction: EdgeDirection,
    ) -> io::Result<()> {
        self.render_with_symbols(w, node_index, direction, &Symbols::default())
    }

    /// Render the dependency graph for the given [`NodeIndex`] using the
    /// provided set of [`Symbols`].
    pub fn render_with_symbols(
        &self,
        w: &mut impl io::Write,
        node_index: NodeIndex,
        direction: EdgeDirection,
        symbols: &Symbols,
    ) -> io::Result<()> {
        Presenter::new(&self.graph, symbols).print_node(w, node_index, direction)
    }

    /// Get the `petgraph` dependency graph.
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Get the nodes of the `petgraph` dependency graph.
    pub fn nodes(&self) -> &Nodes {
        &self.nodes
    }

    /// Add the dependencies of a given package to the given node
    fn add_dependencies<'a>(
        &mut self,
        lockfile: &'a Lockfile,
        package: &'a Package,
        parent: NodeIndex,
        indexed: &mut Set<&'a Package>,
    ) {
        for dependent_package in lockfile.dependent_packages(package) {
            if !indexed.insert(package) {
                continue;
            }

            let dependency = Dependency::from(dependent_package.clone());
            let node_index = self.graph.add_node(dependent_package.clone());
            self.nodes.insert(dependency.clone(), node_index);
            self.graph.add_edge(parent, node_index, dependency);
            self.add_dependencies(lockfile, dependent_package, node_index, indexed);
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
        w: &mut impl io::Write,
        node_index: NodeIndex,
        direction: EdgeDirection,
    ) -> io::Result<()> {
        let package = &self.graph[node_index];
        let new = self.visited.insert(node_index);

        if let Some((&last_continues, rest)) = self.levels_continue.split_last() {
            for &continues in rest {
                let c = if continues { self.symbols.down } else { " " };
                write!(w, "{}   ", c)?;
            }

            let c = if last_continues {
                self.symbols.tee
            } else {
                self.symbols.ell
            };

            write!(w, "{0}{1}{1} ", c, self.symbols.right)?;
        }

        writeln!(w, "{} {}", &package.name, &package.version)?;

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
            self.print_node(w, *dependency, direction)?;
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
