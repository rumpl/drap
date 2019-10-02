use clap::{App, ArgMatches};

pub fn builtin() -> App<'static, 'static> {
  App::new("bundle").subcommands(vec![list::cli()])
}

pub fn exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  let f = match cmd.subcommand() {
    ("list", Some(_)) => list::exec,
    (_, _) => return None,
  };
  Some(f)
}

pub mod list;
