mod rustc_target_info;

fn main() {
    let triples = rustc_target_info::target_triples();
    let targets_info = rustc_target_info::targets_info(&triples);
    for (triple, info) in triples.iter().zip(targets_info) {
        println!("{{
    target_triple: \"{}\",
    target_arch: \"{}\",
    target_os: \"{}\",
    target_env: \"{}\",
    target_endian: \"{}\",
    target_pointer_width: \"{}\",
}};
", &triple, info["target_arch"], info["target_os"], info["target_env"], info["target_endian"], info["target_pointer_width"]);
    }
}

