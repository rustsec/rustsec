//! Data structure to information on a target package
//!
//! Contains fields for name, version, source of the target package, aswell
//! as a field for the pacakge if found, and a field for the identifiers provided 
//! on cli by the user.

use crate::{
    Lockfile,
    package::Name,
    package::Package, error,
};
use serde::{Deserialize, Serialize};
use semver::Version;
use url::Url;
use error::{Result, ErrorKind, Error};

/// Contains the information of target package
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct TargetPackageInfo {
    /// The name of the target package
    pub name: Name,

    /// The version of the target package
    pub version: Option<Version>,

    /// The url of the source of the target package
    pub url: Option<Url>,

    /// The target package (if found)
    pub package: Option<Package>,

    /// The identifiers provided on cli to identify the target package
    pub cli_identifiers: String
}

impl TargetPackageInfo {
    /// Create a new TargetPackageInfo, package field is None
    pub fn new(name: Name, version: Option<Version>, url: Option<Url>, cli_identifiers: String) -> Self {
        Self {
            name,
            version,
            url,
            package: None,
            cli_identifiers
        }
    }

    /// Search a given lockfile for the target package. 
    /// Error if a matching package cannot be found, or multiple packages match.
    pub fn find_target_package (&self, lockfile: &Lockfile) -> Result<Option<Package>> {
        let maybe_targets = TargetPackageInfo::filter_name(self, lockfile)?;
    
        match maybe_targets {
            | None => unreachable!(),
            | Some(targets) if targets.len() == 1 => Ok(Some(targets[0].clone())),
            | Some(targets) => match self.version {
                | None => TargetPackageInfo::filter_url(self, targets),
                | Some(_) => TargetPackageInfo::filter_version(self, targets)
            }
        }
    }
    
    /// Filters a lockfile for packages matching target package name.
    fn filter_name<'a> (&self, lockfile: &'a Lockfile) -> Result<Option<Vec<&'a Package>>> {
        let target_packages:Vec<&Package> = lockfile.packages
            .iter()
            .filter(|package| package.name == self.name)
            .collect();
        
        match target_packages.is_empty() {
            | true => Err(Error::new(ErrorKind::NoTargetFound, &self.cli_identifiers)),
            | false => Ok(Some(target_packages))
        }
    }
    
    /// Filters a list of packages for a package matching the target package version, and url if multiple match version.
    fn filter_version (&self, targets: Vec<&Package>) -> Result<Option<Package>> {
        match self.version.clone() {
            // None arm should never be accessed with current flow
            | None => Err(Error::new(ErrorKind::MultipleTargetsFound, &format!("{}\n{}", self.cli_identifiers, TargetPackageInfo::print_ids(targets)))),
            | Some(version) => {
                let filtered_targets:Vec<&Package> = targets
                    .into_iter()
                    .filter(|package| package.version == version)
                    .collect();
            
                match filtered_targets.len() {
                    | 0 => Err(Error::new(ErrorKind::NoTargetFound, &self.cli_identifiers.to_string())),
                    | 1 => Ok(Some(filtered_targets[0].clone())),
                    | _ => TargetPackageInfo::filter_url(self, filtered_targets),
                }
            }
        }
    }
    
    /// Filters a list of packages for a package matching the target package url, and url if mul.
    fn filter_url (&self, targets: Vec<&Package>) -> Result<Option<Package>> {
        match self.url.clone() {
            | None => Err(Error::new(ErrorKind::MultipleTargetsFound, &format!("{}\n{}", self.cli_identifiers, TargetPackageInfo::print_ids(targets)))),
            | Some(url) => {
                let filtered_targets:Vec<&Package> = targets
                    .into_iter()
                    .filter(|package| package.source.is_some() && *package.source.clone().unwrap().url() == url)
                    .collect();
            
                match filtered_targets.len() {
                    | 0 => Err(Error::new(ErrorKind::NoTargetFound, &self.cli_identifiers.to_string())),
                    | 1 => Ok(Some(filtered_targets[0].clone())),
                    | _ => Err(Error::new(ErrorKind::MultipleTargetsFound, &format!("{}\n{}", self.cli_identifiers, TargetPackageInfo::print_ids(filtered_targets)))),
                }
            }
        }
    }
    
    /// Prints the ids of a list of packages in a format that is target package cli conformant
    fn print_ids(packages: Vec<&Package>) -> String {
        let mut msg = "These packages were found that could match:".to_owned();
    
        for package in packages {
            msg.push_str("\n\t");
        
            // Url of package
            if package.source.is_some() {
                msg.push_str(package.source.as_ref().unwrap().url().as_str());
                msg.push('#')
            }
        
            // Name of package
            msg.push_str(package.name.as_str());
            msg.push('@');
        
            // Version of package
            msg.push_str(package.version.to_string().as_str());
            
        }    
        msg.to_string()
    }
}