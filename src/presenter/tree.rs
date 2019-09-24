//! Dependency tree presenter
//!
//! Includes code from `cargo-tree`, Copyright (c) 2015-2016 Steven Fackler
//! Licensed under the same terms as `cargo-audit` (i.e. Apache 2.0 + MIT)

use rustsec::cargo_lock::{
    dependency_graph::{
        graph::Graph,
        petgraph::{graph::NodeIndex, visit::EdgeRef, EdgeDirection},
    },
    package,
};
use std::collections::BTreeSet as Set;

/// Symbols to use when printing the dependency tree
struct Symbols {
    down: &'static str,
    tee: &'static str,
    ell: &'static str,
    right: &'static str,
}

/// Dependency tree presenter
pub struct Tree<'graph> {
    /// Dependency graph being displayed
    graph: &'graph Graph,

    /// Are there continuing levels?
    levels_continue: Vec<bool>,

    /// Nodes we've already visited
    visited: Set<package::Release>,

    /// Symbols to use when displaying the tree
    symbols: Symbols,
}

impl<'graph> Tree<'graph> {
    /// Print the inverse dependency tree for the given package
    pub fn new(graph: &'graph Graph) -> Self {
        // TODO(tarcieri): support customization of these symbols?
        let symbols = Symbols {
            down: "│",
            tee: "├",
            ell: "└",
            right: "─",
        };

        Self {
            graph,
            levels_continue: vec![],
            visited: Set::new(),
            symbols,
        }
    }

    /// Print a node in the dependency tree
    pub fn print_node(&mut self, node: NodeIndex) {
        let package = &self.graph[node];
        let new = self.visited.insert(package.release());

        if let Some((&last_continues, rest)) = self.levels_continue.split_last() {
            for &continues in rest {
                let c = if continues { self.symbols.down } else { " " };
                print!("{}   ", c);
            }

            let c = if last_continues {
                self.symbols.tee
            } else {
                self.symbols.ell
            };

            print!("{0}{1}{1} ", c, self.symbols.right);
        }

        println!("{} {}", &package.name, &package.version);

        if !new {
            return;
        }

        let dependencies = self
            .graph
            .edges_directed(node, EdgeDirection::Incoming)
            .map(|edge| edge.source())
            .collect::<Vec<_>>();

        for (i, dependency) in dependencies.iter().enumerate() {
            self.levels_continue.push(i < (dependencies.len() - 1));
            self.print_node(*dependency);
            self.levels_continue.pop();
        }
    }
}
