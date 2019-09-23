//! Presenter for `rustsec::Vulnerability` information.
//!
//! Bits and pieces taken from `cargo-tree`, Copyright (c) 2015-2016 Steven Fackler
//! Licensed under the same terms as `cargo-audit` (i.e. Apache 2.0 + MIT)

use abscissa_core::terminal::{
    self,
    Color::{Red, Yellow},
};
use rustsec::{
    cargo_lock::{
        dependency_graph::petgraph::{graph::NodeIndex, visit::EdgeRef, EdgeDirection},
        package, DependencyGraph, Lockfile, Package,
    },
    Vulnerability, Warning,
};
use std::collections::BTreeSet as Set;

/// Symbols which make up the tree
struct TreeSymbols {
    down: &'static str,
    tee: &'static str,
    ell: &'static str,
    right: &'static str,
}

/// UTF-8 representations of these symbols
const DEFAULT_TREE_SYMBOLS: TreeSymbols = TreeSymbols {
    down: "│",
    tee: "├",
    ell: "└",
    right: "─",
};

/// Vulnerability information presenter
#[derive(Clone, Debug)]
pub struct Presenter {
    /// Track packages we've displayed once so we don't show the same dep tree
    // TODO(tarcieri): group advisories about the same package?
    displayed_packages: Set<package::Release>,

    /// Dependency graph for the current workspace
    dependency_graph: DependencyGraph,

    /// Display inverse dependency trees for each vulnerability
    show_dependency_tree: bool,
}

impl Presenter {
    /// Create a new vulnerability information presenter
    pub fn new(lockfile: &Lockfile, show_dependency_tree: bool) -> Self {
        Self {
            displayed_packages: Set::new(),
            dependency_graph: DependencyGraph::new(lockfile).expect("invalid Cargo.lock file"),
            show_dependency_tree,
        }
    }

    /// Print information about the given vulnerability
    pub fn print_vulnerability(&mut self, vulnerability: &Vulnerability) {
        let advisory = &vulnerability.advisory;
        let show_dependency_tree = self
            .displayed_packages
            .insert(vulnerability.package.release())
            && self.show_dependency_tree;

        println!();
        display_attr(Red, "ID:      ", advisory.id.as_str());
        display_attr(Red, "Crate:   ", vulnerability.package.name.as_str());
        display_attr(Red, "Version: ", &vulnerability.package.version.to_string());
        display_attr(Red, "Date:    ", advisory.date.as_str());

        if let Some(url) = advisory.id.url() {
            display_attr(Red, "URL:     ", &url);
        } else if let Some(url) = advisory.url.as_ref() {
            display_attr(Red, "URL:     ", url);
        }

        display_attr(Red, "Title:   ", &advisory.title);
        display_attr(
            Red,
            "Solution: upgrade to",
            &vulnerability
                .versions
                .patched
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .as_slice()
                .join(" OR "),
        );

        if show_dependency_tree {
            terminal::status::Status::new()
                .bold()
                .color(Red)
                .status("Dependency tree:")
                .print_stdout("")
                .unwrap();

            self.print_tree(&vulnerability.package);
        }
    }

    /// Print information about a given warning
    pub fn print_warning(&self, warning: &Warning) {
        println!();

        display_attr(Yellow, "Crate:   ", warning.package.as_str());
        display_attr(Red, "Message: ", warning.message.as_str());

        if let Some(url) = &warning.url {
            display_attr(Yellow, "URL:     ", url);
        }
    }

    /// Print the inverse dependency tree to standard output
    fn print_tree(&self, package: &Package) {
        let mut levels_continue = vec![];
        let mut visited = Set::new();
        let root = self.dependency_graph.nodes()[&package.release()];
        self.print_tree_node(root, &mut visited, &mut levels_continue);
    }

    /// Print a node in the dependency tree
    fn print_tree_node(
        &self,
        node: NodeIndex,
        visited: &mut Set<package::Release>,
        levels_continue: &mut Vec<bool>,
    ) {
        let package = &self.dependency_graph.graph()[node];
        let new = visited.insert(package.release());

        if let Some((&last_continues, rest)) = levels_continue.split_last() {
            for &continues in rest {
                let c = if continues {
                    DEFAULT_TREE_SYMBOLS.down
                } else {
                    " "
                };
                print!("{}   ", c);
            }

            let c = if last_continues {
                DEFAULT_TREE_SYMBOLS.tee
            } else {
                DEFAULT_TREE_SYMBOLS.ell
            };

            print!("{0}{1}{1} ", c, DEFAULT_TREE_SYMBOLS.right);
        }

        println!("{} {}", &package.name, &package.version);

        if !new {
            return;
        }

        let dependencies = self
            .dependency_graph
            .graph()
            .edges_directed(node, EdgeDirection::Incoming)
            .map(|edge| edge.source())
            .collect::<Vec<_>>();

        for (i, dependency) in dependencies.iter().enumerate() {
            levels_continue.push(i < (dependencies.len() - 1));
            self.print_tree_node(*dependency, visited, levels_continue);
            levels_continue.pop();
        }
    }
}

/// Display an attribute of a particular vulnerability
fn display_attr(color: terminal::Color, attr: &str, content: &str) {
    terminal::status::Status::new()
        .bold()
        .color(color)
        .status(attr)
        .print_stdout(content)
        .unwrap();
}
