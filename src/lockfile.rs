use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use toml;

// TODO: Use serde or cargo's builtin types
#[derive(Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
}

pub fn load(filename: &str) -> Result<Vec<Package>, io::Error> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;
    let mut body = String::new();

    file.read_to_string(&mut body).expect("Error reading lockfile!");

    let toml = body.parse::<toml::Value>().expect("Couldn't parse the lockfile!");
    let packages = match toml.get("package") {
        Some(&toml::Value::Array(ref arr)) => arr,
        _ => return Ok(Vec::new())
    };

    Ok(packages.iter()
        .map(|package| {
            Package {
                name: String::from(package["name"].as_str().expect("missing package name!")),
                version: String::from(package["version"]
                    .as_str()
                    .expect("missing package version!")),
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use lockfile;

    #[test]
    fn load_cargo_lockfile() {
        lockfile::load("Cargo.lock").unwrap();
    }
}
