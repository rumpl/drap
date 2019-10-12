use clap::{App, ArgMatches};

pub fn cli() -> App<'static, 'static> {
  App::new("build").about("Build an application")
}

pub fn exec(_: &ArgMatches<'_>) -> Option<fn()> {
  Some(build)
}

fn build() {
  println!("yo build")
}
