use object::{Object, ObjectSymbol};
use rustsec::advisory::affected::FunctionPath;
use std::collections::HashSet;
use syn::TypePath;

pub(crate) type SymbolSet = HashSet<Vec<syn::Ident>>;

/// Extract and demangle all symbols from a binary.
pub(crate) fn demangled_symbols(
    contents: &[u8],
    crate_names: &HashSet<String>,
) -> Option<SymbolSet> {
    let file = object::File::parse(contents).ok()?;
    // `syn::parse_str::<TypePath>` is expensive. The filter on `crate_names`
    // eliminates symbols that we know would be irrelevant.
    Some(
        file.symbols()
            .filter_map(|sym| sym.name().ok())
            .filter(|name| {
                crate_names
                    .iter()
                    .any(|crate_name| name.contains(crate_name.as_str()))
            })
            .map(|name| format!("{:#}", rustc_demangle::demangle(name)))
            .filter_map(|name| syn::parse_str::<TypePath>(&name).ok())
            .map(|type_path| flatten_type_path(&type_path))
            .collect(),
    )
}

fn flatten_type_path(type_path: &TypePath) -> Vec<syn::Ident> {
    let mut idents = Vec::new();
    if let Some(qself) = &type_path.qself
        && let syn::Type::Path(inner) = &*qself.ty
    {
        idents.extend(flatten_type_path(inner));
    }

    for segments in &type_path.path.segments {
        idents.push(segments.ident.clone());
    }
    idents
}

/// Return the affected function paths that appear in the binary's symbol table based on a vulnerability.
pub(crate) fn affected_functions_in_binary_from_vulnerability(
    vulnerability: &rustsec::Vulnerability,
    symbols: &SymbolSet,
) -> Vec<FunctionPath> {
    let Some(function_paths) = vulnerability.affected_functions() else {
        return vec![];
    };
    filter_by_symbols(function_paths, symbols)
}

/// Return the affected function paths that appear in the binary's symbol table based on a warning.
pub(crate) fn affected_functions_in_binary_from_warning(
    warning: &rustsec::Warning,
    symbols: &SymbolSet,
) -> Vec<FunctionPath> {
    let Some(affected) = &warning.affected else {
        return vec![];
    };
    if affected.functions.is_empty() {
        return vec![];
    }
    let function_paths: Vec<_> = affected
        .functions
        .iter()
        .filter(|(_path, version_reqs)| {
            version_reqs
                .iter()
                .any(|req| req.matches(&warning.package.version))
        })
        .map(|(path, _)| path.clone())
        .collect();
    filter_by_symbols(function_paths, symbols)
}

fn filter_by_symbols(function_paths: Vec<FunctionPath>, symbols: &SymbolSet) -> Vec<FunctionPath> {
    function_paths
        .into_iter()
        .filter(|function_path| {
            symbols
                .iter()
                .any(|symbol| function_path_matches_symbol(function_path, symbol))
        })
        .collect()
}

fn function_path_matches_symbol(
    function_path: &FunctionPath,
    symbol_idents: &[syn::Ident],
) -> bool {
    let function_path_segs: Vec<_> = function_path
        .iter()
        .map(|ident| remove_function_path_parameters(ident.as_str()))
        .collect();
    if function_path_segs.is_empty() || symbol_idents.is_empty() {
        return function_path_segs.is_empty() && symbol_idents.is_empty();
    }
    // First segments must match (crate name).
    if symbol_idents[0] != function_path_segs[0] {
        return false;
    }
    // Last segments must match (function name).
    if symbol_idents[symbol_idents.len() - 1] != function_path_segs[function_path_segs.len() - 1] {
        return false;
    }
    // In between the first and last segments, the function path segments must
    // be a subsequence of the symbol segments.
    let function_path_middle = &function_path_segs[1..function_path_segs.len() - 1];
    let symbol_middle = &symbol_idents[1..symbol_idents.len() - 1];
    is_subsequence(function_path_middle, symbol_middle)
}

fn remove_function_path_parameters(ident: &str) -> &str {
    if let Some(n) = ident.as_bytes().iter().position(|&x| x == b'<') {
        &ident[..n]
    } else {
        ident
    }
}

fn is_subsequence(function_path_segs: &[&str], symbol_segs: &[syn::Ident]) -> bool {
    let mut symbol_iter = symbol_segs.iter();
    for function_path_seg in function_path_segs {
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
