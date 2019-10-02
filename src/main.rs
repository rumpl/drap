use clap::App;

fn main() {
    let matches = App::new("drap")
        .version("1.0")
        .author("Djordje Lukic <lukic.djordje@gmail.com")
        .subcommands(drap::commands::builtin())
        .get_matches();

    match drap::commands::builtin_exec(&matches) {
        Some(f) => f(),
        None => println!("unknown command"),
    };
}
