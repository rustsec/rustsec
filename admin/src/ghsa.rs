//! GHSA import prototype
#![allow(unused_variables)] //TODO
#![allow(unused_imports)] //TODO
#![allow(dead_code)] //TODO
#![allow(missing_docs)] //TODO

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use crates_index::Index;
use rustsec::{advisory::Id, Advisory, Database, Repository};

use crate::{
    error::{Error, ErrorKind},
    prelude::*,
};

use serde::Deserialize;
use ureq;

const INITIAL_QUERY: &str = "
  {
    securityVulnerabilities(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}, ecosystem: RUST) {
      nodes {
        advisory {
          publishedAt
          updatedAt
          withdrawnAt
          references {
            url
          }
          identifiers {
            value
          }
          permalink
          ghsaId
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
  }
";

#[derive(Debug, Deserialize)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct GhsaAdvisory {
    #[serde(rename = "publishedAt")]
    published_at: String, //TODO: date
    #[serde(rename = "updatedAt")]
    updated_at: String, //TODO: date
    #[serde(rename = "withdrawnAt")]
    withdrawn_at: Option<String>, //TODO: date
    #[serde(rename = "ghsaId")]
    ghsa_id: String,
    permalink: String,
    references: Vec<Url>,
    identifiers: Vec<Identifier>,
}

// The below structs are implementation details and do not carry any useful data

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct Node {
    advisory: GhsaAdvisory,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct PageInfo {
    endCursor: String,
    hasNextPage: bool,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct SecurityVulnerabilities {
    pageInfo: PageInfo,
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct Data {
    securityVulnerabilities: SecurityVulnerabilities,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

pub struct GhsaImporter {
    /// Loaded advisory DB repository
    repo: Repository,

    /// Loaded Advisory DB
    advisory_db: Database,
}

impl GhsaImporter {
    pub fn new(repo_path: Option<PathBuf>) -> Result<Self, Error> {
        let repo = match repo_path {
            Some(path) => Repository::open(path)?,
            None => Repository::fetch_default_repo()?, //TODO: check if modifying causes issues for `cargo audit`
        };
        let advisory_db = Database::load_from_repo(&repo)?;
        Ok(Self { repo, advisory_db })
    }

    pub fn advisory_for_url(&self, url: &Url) -> Option<&Advisory> {
        let url = &url.url;
        if url.starts_with("https://rustsec.org/advisories/RUSTSEC") {
            let prefix_len = "https://rustsec.org/advisories/".len();
            let id_len = "RUSTSEC-0000-0000".len();
            let id: String = url.chars().skip(prefix_len).take(id_len).collect();
            match Id::from_str(&id) {
                Ok(id) => self.advisory_db.get(&id),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn do_stuff(&self, token: &str) {
        loop {
            let response: Response = graphql_request(INITIAL_QUERY, token).into_json().unwrap();
            let data = response.data.securityVulnerabilities;
            for node in data.nodes {
                self.process_node(node);
            }
            if !data.pageInfo.hasNextPage {
                break;
            }
        }
    }

    fn process_node(&self, node: Node) {
        let advisory = node
            .advisory
            .references
            .iter()
            .find_map(|url| self.advisory_for_url(url));
        if let Some(advisory) = advisory {
            println!(
                "Found match for {}: {}",
                node.advisory.ghsa_id,
                advisory.id()
            );
        } else {
            println!("No match found for {}", node.advisory.ghsa_id);
        }
    }
}

fn graphql_request(request: &str, token: &str) -> ureq::Response {
    ureq::post("https://api.github.com/graphql")
        .set("Authorization", &("bearer ".to_owned() + token))
        .send_json(ureq::json!({
            "query": request,
        }))
        .unwrap() // TODO
}

pub fn fetch_one_page(token: &str) {
    let response: Response = graphql_request(INITIAL_QUERY, token).into_json().unwrap();
    dbg!(response.data);
}
