use clap::App;
use clap::ArgMatches;

pub mod image;

pub fn builtin() -> Vec<App<'static, 'static>> {
  vec![image::builtin()]
}

pub fn builtin_exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  match cmd.subcommand() {
    ("image", Some(submatch)) => image::exec(submatch),
    _ => None,
  }
}
