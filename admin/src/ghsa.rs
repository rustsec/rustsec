//! GHSA import prototype
#![allow(unused_variables)] //TODO
#![allow(unused_imports)] //TODO
#![allow(dead_code)] //TODO
#![allow(missing_docs)] //TODO

use std::path::{Path, PathBuf};

use crates_index::Index;
use rustsec::{Database, Repository};

use crate::{
    error::{Error, ErrorKind},
    prelude::*,
};

use ureq;
use serde::Deserialize;

const QUERY: &str = "
{
    securityAdvisories(first: 100, orderBy: {field: UPDATED_AT, direction: DESC}) {
      pageInfo {
        startCursor
        endCursor
        hasNextPage
      }
      nodes {
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
  }
";

#[derive(Debug, Deserialize)]
pub struct Identifier {
    pub value: String
}

#[derive(Debug, Deserialize)]
pub struct Url {
    pub url: String
}

#[derive(Debug, Deserialize)]
pub struct Node {
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
struct PageInfo {
    startCursor: String,
    endCursor: String,
    hasNextPage: bool
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct SecurityAdvisories {
    pageInfo: PageInfo,
    nodes: Vec<Node>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)] // needs to map directly to GHSA return format
struct Data {
    securityAdvisories: SecurityAdvisories
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data
}

fn graphql_request(request: &str, token: &str) -> ureq::Response {
    ureq::post("https://api.github.com/graphql")
      .set("Authorization", &("bearer ".to_owned() + token))
      .send_json(ureq::json!({
          "query": request,
      })).unwrap() // TODO
}

pub struct GhsaImporter {
    /// Loaded Advisory DB
    advisory_db: Database,
}

impl GhsaImporter {
    pub fn new(repo_path: Option<PathBuf>) -> Result<Self, Error> {
        let repository = match repo_path {
            Some(path) => Repository::open(path)?,
            None => Repository::fetch_default_repo()?,
        };
        let advisory_db = Database::load_from_repo(&repository)?;    
        Ok(Self {
            advisory_db
        })
    }
}

pub fn fetch_stuff(token: &str) {
    let response: Response = graphql_request(QUERY, token).into_json().unwrap();
    dbg!(response.data);
}