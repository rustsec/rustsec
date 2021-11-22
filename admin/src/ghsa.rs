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
use serde_json;
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

/// Same as INITIAL_QUERY, but with an 'after' field.
fn follow_up_query(cursor: &str) -> String {
    // Sadly this has to be a function because the format! macro doesn't like all the { braces } in the query
    "{
      securityVulnerabilities(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}, ecosystem: RUST, after: ".to_owned() +
      serde_json::to_string(cursor).unwrap().as_str() // serializing to JSON sanitizes the input string, just in case
      + ") {
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
  "
}

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
        let response: Response = graphql_request(INITIAL_QUERY, token).into_json().unwrap();
        let data = response.data.securityVulnerabilities;
        for node in data.nodes {
            self.process_ghsa_advisory(node.advisory);
        }
        let mut cursor = data.pageInfo.endCursor;
        let mut has_next_page = data.pageInfo.hasNextPage;
        while has_next_page {
            let query = follow_up_query(&cursor);
            let response_str = graphql_request(&query, token).into_string().unwrap();
            let response: Response = serde_json::from_str(&response_str).unwrap_or_else(|_|{
                println!("{}", &response_str);
                panic!("Oh no")
            });
            //let response: Response = graphql_request(&query, token).into_json().unwrap();
            let data = response.data.securityVulnerabilities;
            for node in data.nodes {
                self.process_ghsa_advisory(node.advisory);
            }
            cursor = data.pageInfo.endCursor;
            has_next_page = data.pageInfo.hasNextPage;
        }
    }

    fn process_ghsa_advisory(&self, gsha_advisory: GhsaAdvisory) {
        let advisory = gsha_advisory
            .references
            .iter()
            .find_map(|url| self.advisory_for_url(url));
        if let Some(advisory) = advisory {
            println!(
                "Found match for {}: {}",
                gsha_advisory.ghsa_id,
                advisory.id()
            );
        } else {
            println!("No match found for {}", gsha_advisory.ghsa_id);
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
