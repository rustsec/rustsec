//! Git repositories

use super::{Commit, DEFAULT_URL};
use crate::{
    error::{Error, ErrorKind},
    fs,
};
use std::path::{Path, PathBuf};

/// Directory under `~/.cargo` where the advisory-db repo will be kept
const ADVISORY_DB_DIRECTORY: &str = "advisory-db";

/// Refspec used to fetch updates from remote advisory databases
const REF_SPEC: &str = "+HEAD:refs/remotes/origin/HEAD";

/// The direction of the remote
const DIR: gix::remote::Direction = gix::remote::Direction::Fetch;

/// Git repository for a Rust advisory DB.
#[cfg_attr(docsrs, doc(cfg(feature = "git")))]
pub struct Repository {
    /// Repository object
    pub(super) repo: gix::Repository,
}

impl Repository {
    /// Location of the default `advisory-db` repository for crates.io
    pub fn default_path() -> PathBuf {
        home::cargo_home()
            .unwrap_or_else(|err| {
                panic!("Error locating Cargo home directory: {}", err);
            })
            .join(ADVISORY_DB_DIRECTORY)
    }

    /// Fetch the default repository
    pub fn fetch_default_repo() -> Result<Self, Error> {
        Self::fetch(DEFAULT_URL, Repository::default_path(), true)
    }

    /// Create a new [`Repository`] with the given URL and path
    pub fn fetch<P: Into<PathBuf>>(
        url: &str,
        into_path: P,
        ensure_fresh: bool,
    ) -> Result<Self, Error> {
        if !url.starts_with("https://") {
            fail!(
                ErrorKind::BadParam,
                "expected {} to start with https://",
                url
            );
        }

        let path = into_path.into();

        if let Some(parent) = path.parent() {
            if !parent.is_dir() {
                fs::create_dir_all(parent)?;
            }
        } else {
            fail!(ErrorKind::BadParam, "invalid directory: {}", path.display())
        }

        // Avoid libgit2 errors in the case the directory exists but is
        // otherwise empty.
        //
        // See: https://github.com/RustSec/cargo-audit/issues/32
        if path.is_dir() && fs::read_dir(&path)?.next().is_none() {
            fs::remove_dir(&path)?;
        }

        // Locking is disabled due to issues with Ctrl+C leaving a stale lockfile,
        // and gix signal handlers not being sophisticated enough
        // to clean up the lockfile in a multi-threaded environment:
        // https://github.com/rustsec/rustsec/pull/934#discussion_r1288376326
        // TODO: re-enable it once `gix` allows registering/unregistering multiple times.
        //
        // // Set up signal handlers so that the lock is released if the user presses Ctrl+C
        // // see https://github.com/rustsec/rustsec/pull/925#discussion_r1287265212
        // gix::interrupt::init_handler(|| {}).unwrap().auto_deregister();
        //
        // // Lock the directory with the git repo checkout so that several processes don't trample each other
        // const LOCK_WAIT_MINUTES: u64 = 10;
        // let _lock = {
        //     let boundary_dir = Some(std::path::PathBuf::from_iter(Some(
        //         std::path::Component::RootDir,
        //     )));
        //     // Attempt to acquire the lock without waiting. If this fails, print a message and wait.
        //     // This is done to avoid `cargo audit` "hanging" without a clearly communicated reason.
        //     match gix::lock::Marker::acquire_to_hold_resource(
        //         path.with_extension("rustsec"),
        //         gix::lock::acquire::Fail::Immediately,
        //         boundary_dir.clone(),
        //     ) {
        //         Ok(marker) => marker,
        //         Err(e) => {
        //             println!("Could not acquire the lock on git repository at {path:?}: {e}. Waiting for up to {LOCK_WAIT_MINUTES} minutes to acquire the lock.");
        //             gix::lock::Marker::acquire_to_hold_resource(
        //                 path.with_extension("rustsec"),
        //                 gix::lock::acquire::Fail::AfterDurationWithBackoff(
        //                     std::time::Duration::from_secs(60 * LOCK_WAIT_MINUTES),
        //                 ),
        //                 boundary_dir,
        //             )
        //             .map_err(|err| {
        //                 format_err!(ErrorKind::Repo, "unable to acquire repo lock: {}", err)
        //             })?
        //         }
        //     }
        // };

        let open_or_clone_repo = || -> Result<_, Error> {
            let mut mapping = gix::sec::trust::Mapping::default();
            let open_with_complete_config =
                gix::open::Options::default().permissions(gix::open::Permissions {
                    config: gix::open::permissions::Config {
                        // Be sure to get all configuration, some of which is only known by the git binary.
                        // That way we are sure to see all the systems credential helpers
                        git_binary: true,
                        ..Default::default()
                    },
                    ..Default::default()
                });

            mapping.reduced = open_with_complete_config.clone();
            mapping.full = open_with_complete_config.clone();

            // Attempt to open the repository, if it fails for any reason,
            // attempt to perform a fresh clone instead
            let repo = gix::ThreadSafeRepository::discover_opts(
                &path,
                gix::discover::upwards::Options::default().apply_environment(),
                mapping,
            )
            .ok()
            .map(|repo| repo.to_thread_local())
            .filter(|repo| {
                repo.find_remote("origin").map_or(false, |remote| {
                    remote
                        .url(DIR)
                        .map_or(false, |remote_url| remote_url.to_bstring() == url)
                })
            })
            .or_else(|| gix::open_opts(&path, open_with_complete_config).ok());

            let res = if let Some(repo) = repo {
                (repo, None)
            } else {
                let mut progress = gix::progress::Discard;
                let should_interrupt = &gix::interrupt::IS_INTERRUPTED;

                let (mut prep_checkout, out) = gix::prepare_clone(url, path)
                    .map_err(|err| {
                        format_err!(ErrorKind::Repo, "failed to prepare clone: {}", err)
                    })?
                    .with_remote_name("origin")
                    .map_err(|err| format_err!(ErrorKind::Repo, "invalid remote name: {}", err))?
                    .configure_remote(|remote| Ok(remote.with_refspecs([REF_SPEC], DIR)?))
                    .fetch_then_checkout(&mut progress, should_interrupt)
                    .map_err(|err| format_err!(ErrorKind::Repo, "failed to fetch repo: {}", err))?;

                let repo = prep_checkout
                    .main_worktree(&mut progress, should_interrupt)
                    .map_err(|err| {
                        format_err!(ErrorKind::Repo, "failed to checkout fresh clone: {}", err)
                    })?
                    .0;

                (repo, Some(out))
            };

            Ok(res)
        };

        let (mut repo, fetch_outcome) = open_or_clone_repo()?;

        if let Some(fetch_outcome) = fetch_outcome {
            tame_index::utils::git::write_fetch_head(
                &repo,
                &fetch_outcome,
                &repo.find_remote("origin").unwrap(),
            )?;
        } else {
            // If we didn't open a fresh repo we need to peform a fetch ourselves, and
            // do the work of updating the HEAD to point at the latest remote HEAD, which
            // gix doesn't currently do.
            Self::perform_fetch(&mut repo)?;
        }

        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        let repo = Self { repo };

        let latest_commit = Commit::from_repo_head(&repo)?;
        latest_commit.reset(&repo)?;

        // Ensure that the upstream repository hasn't gone stale
        if ensure_fresh && !latest_commit.is_fresh() {
            fail!(
                ErrorKind::Repo,
                "repository is stale (last commit: {:?})",
                latest_commit.timestamp
            );
        }

        Ok(repo)
    }

    /// Open a repository at the given path
    pub fn open<P: Into<PathBuf>>(into_path: P) -> Result<Self, Error> {
        let path = into_path.into();
        let repo = gix::open(&path).map_err(|err| {
            format_err!(
                ErrorKind::Repo,
                "failed to open repository at '{}': {}",
                path.display(),
                err
            )
        })?;

        // TODO: Figure out how to detect if the worktree has modifications
        // as gix currently doesn't have a status/state summary like git2 has
        Ok(Self { repo })
    }

    /// Get information about the latest commit to the repo
    pub fn latest_commit(&self) -> Result<Commit, Error> {
        Commit::from_repo_head(self)
    }

    /// Path to the local checkout of a git repository
    pub fn path(&self) -> &Path {
        // Safety: Would fail if this is a bare repo, which we aren't
        self.repo.work_dir().unwrap()
    }

    /// Determines if the tree pointed to by `HEAD` contains the specified path
    pub fn has_relative_path(&self, path: &Path) -> bool {
        let lookup = || {
            self.repo
                .head_commit()
                .ok()?
                .tree()
                .ok()?
                .lookup_entry_by_path(path, &mut Vec::new())
                .ok()
                .map(|_e| true)
        };

        lookup().unwrap_or_default()
    }

    fn perform_fetch(repo: &mut gix::Repository) -> Result<(), Error> {
        let mut config = repo.config_snapshot_mut();
        config
            .set_raw_value("committer", None, "name", "rustsec")
            .map_err(|err| {
                format_err!(ErrorKind::Repo, "failed to set `committer.name`: {}", err)
            })?;
        // Note we _have_ to set the email as well, but luckily gix does not actually
        // validate if it's a proper email or not :)
        config
            .set_raw_value("committer", None, "email", "")
            .map_err(|err| {
                format_err!(ErrorKind::Repo, "failed to set `committer.email`: {}", err)
            })?;

        let repo = config
            .commit_auto_rollback()
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to set `committer`: {}", err))?;

        let mut remote = repo.find_remote("origin").map_err(|err| {
            format_err!(ErrorKind::Repo, "failed to find `origin` remote: {}", err)
        })?;

        remote
            .replace_refspecs(Some(REF_SPEC), DIR)
            .expect("valid statically known refspec");

        // Perform the actual fetch
        let outcome = remote
            .connect(DIR)
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to connect to remote: {}", err))?
            .prepare_fetch(&mut gix::progress::Discard, Default::default())
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to prepare fetch: {}", err))?
            .receive(&mut gix::progress::Discard, &gix::interrupt::IS_INTERRUPTED)
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to fetch: {}", err))?;

        let remote_head_id = tame_index::utils::git::write_fetch_head(&repo, &outcome, &remote)?;

        use gix::refs::{transaction as tx, Target};

        // In all (hopefully?) cases HEAD is a symbolic reference to
        // refs/heads/<branch> which is a peeled commit id, if that's the case
        // we update it to the new commit id, otherwise we just set HEAD
        // directly
        use gix::head::Kind;
        let edit = match repo
            .head()
            .map_err(|err| format_err!(ErrorKind::Repo, "unable to locate HEAD: {}", err))?
            .kind
        {
            Kind::Symbolic(sref) => {
                // Update our local HEAD to the remote HEAD
                if let Target::Symbolic(name) = sref.target {
                    Some(tx::RefEdit {
                        change: tx::Change::Update {
                            log: tx::LogChange {
                                mode: tx::RefLog::AndReference,
                                force_create_reflog: false,
                                message: "".into(),
                            },
                            expected: tx::PreviousValue::MustExist,
                            new: gix::refs::Target::Peeled(remote_head_id),
                        },
                        name,
                        deref: true,
                    })
                } else {
                    None
                }
            }
            Kind::Unborn(_) | Kind::Detached { .. } => None,
        };

        let edit = edit.unwrap_or_else(|| tx::RefEdit {
            change: tx::Change::Update {
                log: tx::LogChange {
                    mode: tx::RefLog::AndReference,
                    force_create_reflog: false,
                    message: "".into(),
                },
                expected: tx::PreviousValue::Any,
                new: gix::refs::Target::Peeled(remote_head_id),
            },
            name: "HEAD".try_into().unwrap(),
            deref: true,
        });

        repo.edit_reference(edit)
            .map_err(|err| format_err!(ErrorKind::Repo, "failed to set update reflog: {}", err))?;

        Ok(())
    }
}
