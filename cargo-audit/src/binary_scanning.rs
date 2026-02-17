use std::collections::HashSet;

use object::{File, Object, ObjectSymbol};
use rustc_demangle::demangle;
use rustsec::{Vulnerability, Warning, advisory::affected::FunctionPath};
use syn::{Ident, Type, TypePath, parse_str};

pub(crate) struct SymbolSet(HashSet<Vec<Ident>>);

impl SymbolSet {
    /// Extract and demangle all symbols from a binary.
    pub(crate) fn from_file(contents: &[u8], crate_names: &HashSet<String>) -> Option<Self> {
        let file = File::parse(contents).ok()?;
        // `parse_str::<TypePath>` is expensive. The filter on `crate_names`
        // eliminates symbols that we know would be irrelevant.
        Some(Self(
            file.symbols()
                .filter_map(|sym| {
                    let name = sym.name().ok()?;
                    if !crate_names
                        .iter()
                        .any(|crate_name| name.contains(crate_name.as_str()))
                    {
                        return None;
                    }
                    let name = format!("{:#}", demangle(name));
                    let type_path = parse_str::<TypePath>(&name).ok()?;
                    Some(flatten_type_path(&type_path))
                })
                .collect(),
        ))
    }

    /// Return the affected function paths that appear in the binary's symbol table based on a vulnerability.
    pub(crate) fn paths_from_vulnerability(
        &self,
        vulnerability: &Vulnerability,
    ) -> impl Iterator<Item = FunctionPath> {
        self.filter(vulnerability.affected_functions().unwrap_or_default())
    }

    /// Return the affected function paths that appear in the binary's symbol table based on a warning.
    pub(crate) fn paths_from_warning(
        &self,
        warning: &Warning,
    ) -> impl Iterator<Item = FunctionPath> {
        self.filter(
            warning
                .affected
                .as_ref()
                .map(|affected| affected.functions.iter())
                .unwrap_or_default()
                .filter_map(|(path, version_reqs)| {
                    if version_reqs
                        .iter()
                        .any(|req| req.matches(&warning.package.version))
                    {
                        Some(path.clone())
                    } else {
                        None
                    }
                }),
        )
    }

    fn filter(
        &self,
        affected: impl IntoIterator<Item = FunctionPath>,
    ) -> impl Iterator<Item = FunctionPath> {
        affected.into_iter().filter(|affected| {
            self.0
                .iter()
                .any(|symbol| function_path_matches_symbol(affected, symbol))
        })
    }
}

fn flatten_type_path(mut type_path: &TypePath) -> Vec<Ident> {
    let mut idents = Vec::new();
    let mut stack = Vec::new();
    loop {
        stack.push(type_path);
        if let Some(qself) = &type_path.qself
            && let Type::Path(inner) = &*qself.ty
        {
            type_path = inner;
            continue;
        } else {
            break;
        }
    }
    while let Some(type_path) = stack.pop() {
        for segment in &type_path.path.segments {
            idents.push(segment.ident.clone());
        }
    }
    idents
}

fn function_path_matches_symbol(affected: &FunctionPath, symbol: &[Ident]) -> bool {
    let affected = affected
        .iter()
        .map(|ident| remove_function_path_parameters(ident.as_str()))
        .collect::<Vec<_>>();
    match (symbol, affected.as_slice()) {
        ([], []) => true,
        ([ident], [affected]) => ident == affected,
        (
            [ident_first, ident_middle @ .., ident_last],
            [affected_first, affected_middle @ .., affected_last],
        ) => {
            // First segments must match (crate name).
            ident_first == affected_first
                // In between the first and last segments, the function path segments must
                // be a subsequence of the symbol segments.
                && is_subsequence(affected_middle, ident_middle)
                // Last segments must match (function name).
                && ident_last == affected_last
        }
        (_, _) => false,
    }
}

fn remove_function_path_parameters(ident: &str) -> &str {
    ident
        .as_bytes()
        .iter()
        .position(|&x| x == b'<')
        .map(|n| &ident[..n])
        .unwrap_or(ident)
}

fn is_subsequence(function_path: &[&str], symbol: &[Ident]) -> bool {
    let mut symbol_iter = symbol.iter();
    for function_path_seg in function_path {
        loop {
            match symbol_iter.next() {
                Some(symbol_seg) if symbol_seg == function_path_seg => break,
                Some(_) => {}
                None => return false,
            }
        }
    }
    true
}
