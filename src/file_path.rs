use crate::types::Template;
use std::path::{Path, PathBuf};

/// Determine where the file path should be written
/// Meant to save people time in dealing with file paths
/// returns (file_path_to_write, file_path_in_top_level)
/// file_path_to_write: filepath to write to disk
/// file_path_in_top_level: filepath to mention in top-level/*.nix
pub fn nix_file_paths(matches: &clap::ArgMatches, template: &Template, path: &std::path::Path, pname: &str) -> (PathBuf, PathBuf) {
    if matches.is_present("nixpkgs") {
        if matches.occurrences_of("pname") == 0 {
            eprintln!("'--pname' is required when using the nixpkgs flag");
            std::process::exit(1);
        }

        if matches.occurrences_of("PATH") == 0 {
            // default to nixpkgs path
            if *template == Template::python {
                let mut radix = PathBuf::from("development/python-modules/");
                radix.push(&pname);
                let mut file_path = PathBuf::from("pkgs");
                file_path.push(&radix);
                file_path.push("default.nix");
                let mut nix_path = PathBuf::from("..");
                nix_path.push(&radix);
                return (file_path.to_path_buf(), nix_path.to_path_buf());
            } else {
                eprintln!("Template '{}' does not have a known path default for the nixpkgs repo, please provide a PATH (E.g. nix-template stdenv --pname mypackage --nixpkgs pkgs/applications/misc/", template);
                std::process::exit(1);
            }
        } else {
            let radix = path.strip_prefix("pkgs").unwrap_or(path);
            let mut file_path = PathBuf::from("pkgs");
            file_path.push(&radix);
            if file_path.extension() != Some(std::ffi::OsStr::new("nix")) {
                file_path.push("default.nix");
            }
            let mut nix_path = PathBuf::from("..");
            nix_path.push(&radix);
            return (file_path.to_path_buf(), nix_path.to_path_buf());
        }
    }
    (path.to_path_buf(), PathBuf::from(""))
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{build_cli, validate_and_serialize_matches};
    use pretty_assertions::{assert_eq};

    #[test]
    fn test_python() {
        let m =
            build_cli().get_matches_from(vec!["nix-template", "python", "-n", "-p", "requests"]);
        let info = validate_and_serialize_matches(&m);
        let expected = (PathBuf::from("pkgs/development/python-modules/requests/default.nix"), PathBuf::from("../development/python-modules/requests"));
        let actual = nix_file_paths(&m, &info.template, &info.path_to_write, &info.pname);
        assert_eq!(expected, actual);
    }
}

