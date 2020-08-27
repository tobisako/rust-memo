use clap::{App, Arg, SubCommand, ArgMatches};

pub fn cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("simulator")
        .version("0.0.1")
        .subcommand(
            SubCommand::with_name("sub")
                .arg(Arg::with_name("test_flag")
                    .short("t")
                        .takes_value(true)
//                        .default_value("true")
                        .help("test flag (default false, no-value-default: false)"))
    )
}

pub fn chk_sub(matches: &ArgMatches) -> Result<(), String> {

    // not work :d
    let is_t = matches.is_present("test_flag");
    println!("is_t={}", is_t);
    if is_t {
        let value = matches.value_of("continue_after_checks").unwrap_or("none");
        println!("value={}", value);
    } else {
        println!("value is nothing (default)");
    }

    // needs case is:
    //  ./clap3 sub -t true  -> detect true.
    //  ./clap3 sub -t false -> detect false.
    //  ./clap3 sub -t       -> detect true.
    //  ./clap3 sub          -> detect false.

    // but I have no idea.

    Ok(())
}

fn main() {
    let matches = cli_app().get_matches();
    match matches.subcommand() {
        ("sub", Some(matches)) => match chk_sub(matches) {
            Ok(()) => println!("[successfully]"),
            Err(e) => {
                eprintln!("error: {}", e);
                std::process::exit(1)
            }
        },
        _ => {
            eprintln!("[Invalid subcommand]");
            std::process::exit(1)
        }
    }
}
