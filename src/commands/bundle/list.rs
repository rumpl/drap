use crate::bundle::store;
use clap::App;

pub fn cli() -> App<'static, 'static> {
  App::new("list")
}

pub fn exec() {
  let bundles = store::list_bundles();
  for bundle in &bundles {
    println!("{}", bundle.name)
  }
}
