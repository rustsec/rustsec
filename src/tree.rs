//! Support for displaying inverse dependency trees ala `cargo-tree -i`
//!
//! Bits and pieces taken from `cargo-tree`, Copyright (c) 2015-2016 Steven Fackler
//! Licensed under the same terms as `cargo-audit` (i.e. Apache 2.0 + MIT)

use rustsec::{
    lockfile::dependency_graph::{
        petgraph::{graph::NodeIndex, visit::EdgeRef, EdgeDirection},
        DependencyGraph,
    },
    package, Lockfile, Package,
};
use std::collections::BTreeSet as Set;

/// Symbols which make up the tree
struct Symbols {
    down: &'static str,
    tee: &'static str,
    ell: &'static str,
    right: &'static str,
}

/// UTF-8 representations of these symbols
const UTF8_SYMBOLS: Symbols = Symbols {
    down: "│",
    tee: "├",
    ell: "└",
    right: "─",
};

/// Dependency tree
#[derive(Clone, Debug)]
pub struct Tree(DependencyGraph);

impl Tree {
    /// Construct a new tree for a particular package
    pub fn new(lockfile: &Lockfile) -> Self {
        Tree(lockfile.dependency_graph())
    }

    /// Print the inverse dependency tree to standard output
    pub fn print(&self, package: &Package) {
        let mut levels_continue = vec![];
        let mut visited = Set::new();
        let root = self.0.nodes()[&package.name];
        self.print_node(root, &mut visited, &mut levels_continue);
    }

    /// Print a node in the dependency tree
    fn print_node(
        &self,
        node: NodeIndex,
        visited: &mut Set<package::Name>,
        levels_continue: &mut Vec<bool>,
    ) {
        let package = &self.0.graph()[node];
        let new = visited.insert(package.name.clone());

        if let Some((&last_continues, rest)) = levels_continue.split_last() {
            for &continues in rest {
                let c = if continues { UTF8_SYMBOLS.down } else { " " };
                print!("{}   ", c);
            }

            let c = if last_continues {
                UTF8_SYMBOLS.tee
            } else {
                UTF8_SYMBOLS.ell
            };

            print!("{0}{1}{1} ", c, UTF8_SYMBOLS.right);
        }

        println!("{} {}", &package.name, &package.version);

        if !new {
            return;
        }

        let dependencies = self
            .0
            .graph()
            .edges_directed(node, EdgeDirection::Incoming)
            .map(|edge| edge.source())
            .collect::<Vec<_>>();

        for (i, dependency) in dependencies.iter().enumerate() {
            levels_continue.push(i < (dependencies.len() - 1));
            self.print_node(*dependency, visited, levels_continue);
            levels_continue.pop();
        }
    }
}
