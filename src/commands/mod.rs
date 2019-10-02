use clap::App;
use clap::ArgMatches;

pub mod bundle;

pub fn builtin() -> Vec<App<'static, 'static>> {
  vec![bundle::builtin()]
}

pub fn builtin_exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  match cmd.subcommand() {
    ("bundle", Some(submatch)) => bundle::exec(submatch),
    _ => None,
  }
}
