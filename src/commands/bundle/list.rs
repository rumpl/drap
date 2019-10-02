use crate::bundle::store::BundleStore;
use clap::App;

pub fn cli() -> App<'static, 'static> {
  App::new("list")
}

pub fn exec() {
  match BundleStore::new(String::from("")) {
    Ok(store) => {
      let bundles = store.list_bundles();
      for bundle in &bundles {
        println!("{}", bundle.name)
      }
    }
    Err(s) => println!("{}", s),
  }
}
