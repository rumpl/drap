use clap::App;
use clap::ArgMatches;

pub mod build;
pub mod image;
pub mod run;

pub fn builtin() -> Vec<App<'static, 'static>> {
  vec![image::builtin(), build::cli(), run::cli()]
}

pub fn builtin_exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  match cmd.subcommand() {
    ("image", Some(submatch)) => image::exec(submatch),
    ("build", Some(submatch)) => build::exec(submatch),
    ("run", Some(submatch)) => run::exec(submatch),
    _ => None,
  }
}
