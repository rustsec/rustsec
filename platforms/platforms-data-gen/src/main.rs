mod rustc_target_info;

fn main() {
    let triples = rustc_target_info::target_triples();
    let targets_info = rustc_target_info::targets_info(&triples);
    for (triple, info) in triples.iter().zip(targets_info) {
        println!("{{
    target_triple: \"{}\",
    target_arch: \"{}\",
    target_os: \"{}\",
    target_env: \"{}\"
}};
", &triple, info["target_arch"], info["target_os"], info["target_env"]);
    }
}

