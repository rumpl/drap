use clap::App;
use clap::ArgMatches;

pub mod build;
pub mod image;

pub fn builtin() -> Vec<App<'static, 'static>> {
  vec![image::builtin(), build::builtin()]
}

pub fn builtin_exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  match cmd.subcommand() {
    ("image", Some(submatch)) => image::exec(submatch),
    ("build", Some(submatch)) => build::exec(submatch),
    _ => None,
  }
}
