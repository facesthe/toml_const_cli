//! Stuff for looking around cargo packages

use std::{fs, path::PathBuf, str::FromStr};

use toml::Value;

/// Cargo manifest feature struct
#[derive(Clone, Debug)]
#[allow(unused)]
struct CargoManifest {
    /// Package name
    package: Option<String>,
    /// If manifest defines a workspace
    workspace: bool,
    /// Binary names, if any
    binaries: Option<Vec<String>>,
    /// Library name, if any
    library: Option<String>,
}

impl FromStr for CargoManifest {
    type Err = String;

    /// Read in the contents of a Cargo.toml file into the struct
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let table = toml::Table::from_str(s).map_err(|e| e.to_string())?;

        let package = table
            .get("package")
            .and_then(|p| p.get("name"))
            .and_then(|name| {
                if let Value::String(n) = name {
                    Some(n.clone())
                } else {
                    None
                }
            });

        let workspace = match table.get("workspace") {
            Some(_) => true,
            None => false,
        };

        let binaries = table.get("bin").and_then(|t| {
            if let Value::Array(bins) = t {
                let bin_names = bins
                    .iter()
                    .map(|inner| {
                        let name = inner
                            .get("name")
                            .expect("each binary target should have a name");

                        if let Value::String(name) = name {
                            name.clone()
                        } else {
                            panic!("binary target should be a string") // this branch should not be taken
                        }
                    })
                    .collect::<Vec<_>>();

                Some(bin_names)
            } else {
                None
            }
        });

        let library = table.get("lib").and_then(|t| match t.get("name") {
            Some(v) => {
                if let Value::String(n) = v {
                    Some(n.clone())
                } else {
                    None
                }
            }
            None => None,
        });

        Ok(Self {
            package,
            workspace,
            binaries,
            library,
        })
    }
}

/// Starting from the current directory, go up a parent until a workspace manifest
/// or a package manifest is found. If a package manifest is found, continue searching
/// until reaching the filesystem root.
pub fn find_cargo_parent(start: &PathBuf) -> Option<PathBuf> {
    let full_path = start.canonicalize().ok()?;

    #[allow(unused_assignments)]
    let mut manifest: Option<CargoManifest> = None;
    let mut path: Option<PathBuf> = None;

    for upper_path in full_path.ancestors() {
        let mut cargo_path = upper_path.to_path_buf();
        cargo_path.push("Cargo.toml");

        if cargo_path.is_file() {
            let file = fs::read_to_string(&cargo_path).ok()?;
            manifest = CargoManifest::from_str(&file).ok();

            match manifest {
                Some(m) => {
                    // println!("{:?}", m);
                    if m.workspace {
                        // break when encountering workspace manifest
                        path = Some(cargo_path.clone());
                        break;
                    } else if let Some(_) = m.package {
                        path = Some(cargo_path.clone())
                    }
                }
                None => (),
            }
        }
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cargo_parent() {
        let curr_path = PathBuf::from(".");

        println!("start: {:?}", curr_path.canonicalize());
        let res = find_cargo_parent(&curr_path);
        assert!(matches!(res, Some(_)));
        println!("{:?}", res);
    }
}
