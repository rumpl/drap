use clap::{App, ArgMatches};

pub fn builtin() -> App<'static, 'static> {
  App::new("image")
    .help_message("Application image management")
    .subcommands(vec![list::cli("list"), list::cli("ls")])
}

pub fn exec(cmd: &ArgMatches<'_>) -> Option<fn()> {
  let f = match cmd.subcommand() {
    ("ls", Some(_)) => list::exec,
    ("list", Some(_)) => list::exec,
    (_, _) => return None,
  };
  Some(f)
}

pub mod list;
