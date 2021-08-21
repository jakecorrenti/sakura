use chrono::prelude::*;
use clap::ArgMatches;
use crossterm::style::Stylize;

mod db;

pub fn run(args: &ArgMatches) {
    match args.subcommand_name() {
        Some("add") => {
            let add_matches = args.subcommand_matches("add").unwrap();

            let name = add_matches.value_of("name").unwrap();
            let due_date = add_matches.value_of("due-date").unwrap();

            let name = name.to_string();
            let due_date = due_date.to_string();

            match verify_due_date(due_date.as_str()) {
                DueDateValidity::Valid(date) => {
                    //TODO(jakecorrenti): Handle the Results for these calls accordingly
                    db::create_table();
                    db::save_new_item(name.as_str(), date.as_str());
                }
                DueDateValidity::Invalid(reason) => {
                    panic!("Date entered is invalid: {}", reason);
                }
            }
        }
        Some("list") => match db::get_all_items() {
            Ok(items) => {
                for item in items {
                    let date_components: Vec<&str> = item.due_date().split('-').collect();

                    let month = date_components[0].to_string().parse::<u32>().unwrap();
                    let day = date_components[1].to_string().parse::<u32>().unwrap();
                    let year = date_components[2].to_string().parse::<i32>().unwrap();

                    if Utc.ymd(year, month, day).lt(&Utc::now().date()) {
                        println!("{}: {}", item.due_date().red(), item.name());
                    } else {
                        println!("{}: {}", item.due_date().green(), item.name());
                    }
                }
            }
            Err(e) => panic!(e),
        },
        _ => (),
    }
}

enum DueDateValidity {
    Valid(String),
    Invalid(String),
}

fn verify_due_date(input: &str) -> DueDateValidity {
    let improper_formatting =
        String::from("The date was improperly formatted. Format must be mm-dd-yyyy");
    let invalid_date =
        String::from("The date provided must be later than or equal to today's date");

    if !input.contains('-') {
        return DueDateValidity::Invalid(improper_formatting);
    }

    let date_components: Vec<&str> = input.split('-').collect();

    if date_components.len() != 3
        || date_components[0].len() != 2
        || date_components[1].len() != 2
        || date_components[2].len() != 4
    {
        return DueDateValidity::Invalid(improper_formatting);
    }

    let month = date_components[0]
        .to_string()
        .parse::<u32>()
        .expect(&improper_formatting);
    let day = date_components[1]
        .to_string()
        .parse::<u32>()
        .expect(&improper_formatting);
    let year = date_components[2]
        .to_string()
        .parse::<i32>()
        .expect(&improper_formatting);

    if Utc.ymd(year, month, day).lt(&Utc::now().date()) {
        return DueDateValidity::Invalid(invalid_date);
    }

    let provided_date = Utc.ymd(year, month, day).format("%m-%d-%Y").to_string();
    DueDateValidity::Valid(provided_date)
}
