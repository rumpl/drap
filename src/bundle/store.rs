use crate::cnab::Bundle;
use dirs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Reference {
    pub name: String,
}

#[derive(Debug)]
pub struct StoreBundle {
    pub reference: Reference,
    pub bundle: Bundle,
}

impl Reference {
    pub fn named(n: &Path) -> Reference {
        Reference {
            name: Reference::parse_named(n.to_str().unwrap()),
        }
    }

    fn parse_named(s: &str) -> String {
        let parts: Vec<&str> = s.split('/').collect();
        // Dirty dirty
        if parts.len() == 6 {
            return format!("{}:{}", parts[2], parts[4].trim_end_matches(".json"));
        }
        parts[1].to_string()
    }
}

pub struct BundleStore {
    path: PathBuf,
}

impl BundleStore {
    pub fn new(_path: String) -> Result<BundleStore, &'static str> {
        match dirs::home_dir() {
            Some(p) => Ok(BundleStore {
                path: p.join(".docker").join("app"),
            }),
            None => Err("unable to find home dir"),
        }
    }

    pub fn list_bundles(&self) -> Vec<StoreBundle> {
        self.get_bundle_files()
    }

    fn get_bundle_files(&self) -> Vec<StoreBundle> {
        WalkDir::new(self.path.join("bundles"))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(|e| StoreBundle {
                reference: Reference::named(
                    e.path().strip_prefix(self.path.join("bundles")).unwrap(),
                ),
                bundle: Bundle::from_file(e.path()).unwrap(),
            })
            .collect()
    }
}
