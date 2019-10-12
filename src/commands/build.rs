use clap::{App, ArgMatches};

pub fn builtin() -> App<'static, 'static> {
  App::new("build").about("Build an application")
}

pub fn exec(_: &ArgMatches<'_>) -> Option<fn()> {
  Some(build)
}

fn build() {
  println!("yo build")
}