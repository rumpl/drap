use clap::App;
use drap::commands;

fn main() {
    let matches = App::new("drap")
        .version("1.0")
        .author("Djordje Lukic <lukic.djordje@gmail.com")
        .subcommands(drap::commands::builtin())
        .get_matches();

    match commands::builtin_exec(&matches) {
        Some(f) => f(),
        None => println!("unknown command"),
    };
}
