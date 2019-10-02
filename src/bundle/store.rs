#[derive(Debug)]
pub struct Bundle {
    pub name: String,
}

pub fn list_bundles() -> Vec<Bundle> {
    let mut ret = Vec::new();
    ret.push(Bundle {
        name: String::from("Test"),
    });
    ret.push(Bundle {
        name: String::from("Another"),
    });
    ret
}
