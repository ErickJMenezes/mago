use std::path::Path;
use std::path::PathBuf;

use ahash::HashSet;
use async_walkdir::Filtering;
use async_walkdir::WalkDir;
use futures::StreamExt;
use std::process::Command;
use std::process::Stdio;

use mago_interner::ThreadedInterner;
use mago_source::SourceCategory;
use mago_source::SourceManager;

use crate::config::source::SourceConfiguration;
use crate::consts::PHP_STUBS;
use crate::error::Error;

const FAILED_GIT_COMMAND: &str = "Failed to execute git command. It's probably not installed or is not in your PATH.";

/// Load the source manager from the given files or directories,
/// ignoring the `paths`, `includes`, and `excludes` configuration.
///
/// # Arguments
///
/// * `interner` - The interner to use for string interning.
/// * `configuration` - The configuration to use for loading the sources.
/// * `files` - The files to load into the source manager.
/// * `include_stubs` - Whether to include stubs in the source manager.
///
/// # Returns
///
/// A `Result` containing the new source manager or a `Error` if
/// an error occurred during the build process.
pub async fn from_paths(
    interner: &ThreadedInterner,
    configuration: &SourceConfiguration,
    paths: Vec<PathBuf>,
    include_stubs: bool,
) -> Result<SourceManager, Error> {
    let SourceConfiguration { root, extensions, .. } = configuration;

    let manager = SourceManager::new(interner.clone());

    let excludes_set = HashSet::default();
    let extensions: HashSet<&str> = extensions.iter().map(|ext| ext.as_str()).collect();

    for path in paths {
        add_path_to_manager(&manager, path, root, &[], &excludes_set, &extensions, true).await?;
    }

    if include_stubs {
        for (stub, content) in PHP_STUBS {
            manager.insert_content(stub, content, SourceCategory::BuiltIn);
        }
    }

    Ok(manager)
}

/// Load the source manager from the modified files in a Git repository.
///
/// This function checks if the current directory is a Git repository,
/// collects the list of modified files using `git diff --name-only`,
/// and processes those files to build a source manager.
///
/// # Arguments
///
/// * `interner` - The interner to use for string interning.
/// * `configuration` - The configuration to use for loading the sources.
/// * `include_externals` - Whether to include external sources in the source manager.
/// * `include_stubs` - Whether to include stubs in the source manager.
///
/// # Returns
///
/// A `Result` containing the new `SourceManager` or an `Error` if an error
/// occurred during the build process.
///
/// # Errors
///
/// This function will return an error if:
/// - The current working directory is not part of a Git repository.
/// - The `git` command is not available or fails to execute.
/// - Any file operation or source processing fails during execution.
pub async fn from_modified_files(
    interner: &ThreadedInterner,
    configuration: &SourceConfiguration,
    include_externals: bool,
    include_stubs: bool,
) -> Result<SourceManager, Error> {
    let paths = git_modified_files();

    if paths.is_empty() {
        // when the `paths` array is empty, there's nothing to fix, lint and format.
        // Just return an empty source manager.
        return Ok(SourceManager::new(interner.clone()));
    }

    let new_configuration = SourceConfiguration {
        root: configuration.root.clone(),
        includes: configuration.includes.clone(),
        paths,
        excludes: configuration.excludes.clone(),
        extensions: configuration.extensions.clone(),
    };

    load(interner, &new_configuration, include_externals, include_stubs).await
}

#[inline]
fn git_modified_files() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();

    let git_output = Command::new("git")
        .args(["--no-pager", "diff", "--name-only"])
        .stderr(Stdio::null())
        .output()
        .expect(FAILED_GIT_COMMAND);

    let output = String::from_utf8_lossy(&git_output.stdout);

    for line in output.lines() {
        let path = Path::new(line);
        paths.push(path.to_path_buf());
    }

    paths
}

/// Check if `git` is available as a command and the current directory is inside a git repository.
///
/// This function verifies whether the current working directory is part of a
/// Git repository by running the `git rev-parse --is-inside-work-tree` command.
///
/// # Returns
///
/// A boolean value:
/// - `true` if the directory is inside a Git repository and `git` is available as a command.
/// - `false` if not.
pub fn is_git_available() -> bool {
    let mut command = Command::new("git");
    command.args(["rev-parse", "--is-inside-work-tree"]).stderr(Stdio::null());

    match command.output() {
        Ok(output) => {
            output.status.success() && String::from_utf8_lossy(&output.stdout).trim().eq_ignore_ascii_case("true")
        }
        Err(_) => false,
    }
}

/// Load the source manager by scanning and processing the sources
/// as per the given configuration.
///
/// # Arguments
///
/// * `interner` - The interner to use for string interning.
/// * `configuration` - The configuration to use for loading the sources.
/// * `include_externals` - Whether to include external sources in the source manager.
/// * `include_stubs` - Whether to include stubs in the source manager.
///
/// # Returns
///
/// A `Result` containing the new source manager or a `Error` if
/// an error occurred during the build process.
pub async fn load(
    interner: &ThreadedInterner,
    configuration: &SourceConfiguration,
    include_externals: bool,
    include_stubs: bool,
) -> Result<SourceManager, Error> {
    let SourceConfiguration { root, paths, includes, excludes, extensions } = configuration;

    let mut starting_paths = Vec::new();

    if paths.is_empty() {
        starting_paths.push((root.to_path_buf(), true));
    } else {
        for source in paths {
            starting_paths.push((source.clone(), true));
        }
    }

    if include_externals {
        for include in includes {
            starting_paths.push((include.clone(), false));
        }
    }

    let excludes_set = create_excludes_set(excludes, root);
    let extensions: HashSet<&str> = extensions.iter().map(|ext| ext.as_str()).collect();

    let manager = SourceManager::new(interner.clone());
    for (path, user_defined) in starting_paths.into_iter() {
        add_path_to_manager(&manager, path, root, includes, &excludes_set, &extensions, user_defined).await?;
    }

    if include_stubs {
        for (stub, content) in PHP_STUBS {
            manager.insert_content(stub, content, SourceCategory::BuiltIn);
        }
    }

    Ok(manager)
}

#[inline(always)]
async fn add_path_to_manager(
    manager: &SourceManager,
    path: PathBuf,
    root: &Path,
    includes: &[PathBuf],
    excludes_set: &HashSet<Exclusion>,
    extensions: &HashSet<&str>,
    user_defined: bool,
) -> Result<(), Error> {
    if !path.exists() || path.is_symlink() {
        return Ok(());
    }

    if !path.is_dir() {
        add_file_to_manager(manager, path, root, includes, excludes_set, extensions, user_defined);

        return Ok(());
    }

    let mut entries = WalkDir::new(path).filter(|entry| async move {
        if entry.path().starts_with(".") { Filtering::IgnoreDir } else { Filtering::Continue }
    });

    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        if path.is_dir() || path.is_symlink() {
            continue;
        }

        add_file_to_manager(manager, path, root, includes, excludes_set, extensions, user_defined);
    }

    Ok(())
}

#[inline(always)]
fn add_file_to_manager(
    manager: &SourceManager,
    path: PathBuf,
    root: &Path,
    includes: &[PathBuf],
    excludes_set: &HashSet<Exclusion>,
    extensions: &HashSet<&str>,
    user_defined: bool,
) {
    // Skip user-defined sources if they are included in the `includes` list.
    if user_defined && includes.iter().any(|include| path.starts_with(include)) {
        return;
    }

    // Skip excluded files and directories.
    if is_excluded(&path, excludes_set) {
        return;
    }

    // Skip files that do not have an accepted extension.
    if path.is_file() && !is_accepted_file(&path, extensions) {
        return;
    }

    let name = match path.strip_prefix(root) {
        Ok(rel_path) => rel_path.display().to_string(),
        Err(_) => path.display().to_string(),
    };

    manager.insert_path(name, path, if user_defined { SourceCategory::UserDefined } else { SourceCategory::External });
}

fn create_excludes_set(excludes: &[String], root: &Path) -> HashSet<Exclusion> {
    excludes
        .iter()
        .map(|exclude| {
            // if it contains a wildcard, treat it as a pattern
            if exclude.contains('*') {
                Exclusion::Pattern(exclude.clone())
            } else {
                let path = Path::new(exclude);

                if path.is_absolute() { Exclusion::Path(path.to_path_buf()) } else { Exclusion::Path(root.join(path)) }
            }
        })
        .collect()
}

fn is_excluded(path: &Path, excludes: &HashSet<Exclusion>) -> bool {
    for exclusion in excludes {
        return match exclusion {
            Exclusion::Path(p) if path.starts_with(p) => true,
            Exclusion::Pattern(p) if glob_match::glob_match(p, path.to_string_lossy().as_ref()) => true,
            _ => continue,
        };
    }

    false
}

fn is_accepted_file(path: &Path, extensions: &HashSet<&str>) -> bool {
    if extensions.is_empty() {
        path.extension().and_then(|s| s.to_str()).map(|ext| ext.eq_ignore_ascii_case("php")).unwrap_or(false)
    } else {
        path.extension().and_then(|s| s.to_str()).map(|ext| extensions.contains(ext)).unwrap_or(false)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Exclusion {
    Path(PathBuf),
    Pattern(String),
}
