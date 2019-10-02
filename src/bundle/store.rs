use dirs;
use std::path::PathBuf;
use walkdir::WalkDir;
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

    pub fn list_bundles(&self) -> Vec<Bundle> {
        let mut ret = Vec::new();
        for entry in WalkDir::new(self.path.join("bundles"))
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            if let Ok(p) = entry.path().strip_prefix(self.path.join("bundles")) {
                ret.push(Bundle {
                    name: p.to_str().unwrap().to_string(),
                })
            };
        }
        ret
    }
}
