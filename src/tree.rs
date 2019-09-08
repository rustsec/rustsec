//! Support for displaying inverse dependency trees ala `cargo-tree -i`
//!
//! Bits and pieces taken from `cargo-tree`, Copyright (c) 2015-2016 Steven Fackler
//! Licensed under the same terms as `cargo-audit` (i.e. Apache 2.0 + MIT)

use petgraph::{
    graph::{Graph, NodeIndex},
    visit::EdgeRef,
    EdgeDirection,
};
use rustsec::{package, Lockfile, Package};
use std::collections::{BTreeMap as Map, BTreeSet as Set};

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

/// Inverse dependency tree for a particular package
// TODO(tarcieri): compute this once for the lockfile and reuse
#[derive(Clone, Debug)]
pub struct Tree {
    /// Dependency graph for a particular package
    graph: Graph<Package, Edge>,

    /// Package data associated with nodes in the tree
    nodes: Map<package::Name, NodeIndex>,

    /// Root of the tree
    root: NodeIndex,
}

impl Tree {
    /// Construct a new tree for a particular package
    pub fn new(lockfile: &Lockfile, package: &Package) -> Self {
        let mut graph = Graph::new();
        let root = graph.add_node(package.clone());

        let mut nodes = Map::new();
        nodes.insert(package.name.clone(), root);

        let mut tree = Self { graph, nodes, root };
        tree.add_dependent_packages(lockfile, package, root);
        tree
    }

    /// Get the nodes of the tree
    pub fn nodes(&self) -> &Map<package::Name, NodeIndex> {
        &self.nodes
    }

    /// Print the tree to standard output
    pub fn print(&self) {
        let mut levels_continue = vec![];
        let mut visited = Set::new();
        self.print_node(self.root, &mut visited, &mut levels_continue);
    }

    /// Print a node in the dependency tree
    fn print_node(
        &self,
        node: NodeIndex,
        visited: &mut Set<package::Name>,
        levels_continue: &mut Vec<bool>,
    ) {
        let package = &self.graph[node];
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
            .graph
            .edges_directed(node, EdgeDirection::Incoming)
            .map(|edge| edge.source())
            .collect::<Vec<_>>();

        for (i, dependency) in dependencies.iter().enumerate() {
            levels_continue.push(i < (dependencies.len() - 1));
            self.print_node(*dependency, visited, levels_continue);
            levels_continue.pop();
        }
    }

    /// Add the dependencies of a given package to the given node
    fn add_dependent_packages(
        &mut self,
        lockfile: &Lockfile,
        package: &Package,
        parent: NodeIndex,
    ) {
        for dependent_package in lockfile.dependent_packages(package) {
            let node = self.graph.add_node(dependent_package.clone());
            self.nodes.insert(dependent_package.name.clone(), node);

            self.graph.add_edge(node, parent, Edge);
            self.add_dependent_packages(lockfile, dependent_package, node);
        }
    }
}

/// Graph edges. These are presently just a placeholder.
// TODO(tarcieri): use these for e.g. VersionReqs?
#[derive(Clone, Debug)]
pub struct Edge;
