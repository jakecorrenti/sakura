use clap::ArgMatches;

pub fn run(args: &ArgMatches) {
    match args.subcommand_name() {
        Some("add") => {
            let add_matches = args.subcommand_matches("add").unwrap();
            println!("{}", add_matches.value_of("name").unwrap());
            println!("{}", add_matches.value_of("due-date").unwrap());
        },
        _ => (),
    }
}
