use dirs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub struct Reference {
    pub name: String,
}

impl Reference {
    pub fn named(n: String) -> Reference {
        Reference {
            name: Reference::parse_named(n),
        }
    }

    fn parse_named(s: String) -> String {
        let parts: Vec<&str> = s.split('/').collect();
        // Dirty dirty
        if parts.len() == 6 {
            return format!("{}:{}", parts[2], parts[4].trim_end_matches(".json"));
        }
        parts[1].to_string()
    }
}

#[derive(Debug)]
pub struct Bundle {
    pub name: String,
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

    pub fn list_bundles(&self) -> Vec<Reference> {
        self.get_bundle_files()
            .into_iter()
            .map(Reference::named)
            .collect()
    }

    fn get_bundle_files(&self) -> Vec<String> {
        WalkDir::new(self.path.join("bundles"))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
            .map(
                |p| match p.into_path().strip_prefix(self.path.join("bundles")) {
                    Ok(f) => Ok(f.to_path_buf()),
                    Err(r) => Err(r),
                },
            )
            .filter_map(Result::ok)
            .map(|e| e.to_str().unwrap().to_string())
            .collect()
    }
}
