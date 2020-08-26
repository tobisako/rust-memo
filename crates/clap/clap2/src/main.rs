
//use clap::ArgMatches;
//use clap::ArgMatches::values_of;

use clap::{App, Arg, SubCommand, value_t, ArgMatches};

pub fn cli_app<'a, 'b>() -> App<'a, 'b> {
    App::new("simulator")
        .version("0.0.1")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Options for interacting with simulator")
        .subcommand(
            SubCommand::with_name("eth1-sim")
            .about(
                "Lighthouse Beacon Chain Simulator ...",
                    )
                    // .arg(Arg::with_name("nodes")
                    //     .short("n")
                    //     .long("nodes")
                    //     .takes_value(true)
                    //     .default_value("4")
                    //     .help("Number of beacon nodes"))
                    // .arg(Arg::with_name("validators_per_node")
                    //     .short("v")
                    //     .long("validators_per_node")
                    //     .takes_value(true)
                    //     .default_value("20")
                    //     .help("Number of validators"))
                    .arg(Arg::with_name("speed_up_factor")
                        .short("s")
                        .long("speed_up_factor")
                        .takes_value(true)
                        .default_value("4")
                        .help("Speed up factor (default 4"))
                    .arg(Arg::with_name("continue_after_checks")
                        .short("c")
                        .long("continue_after_checks")
                        .takes_value(true)
                        .default_value("false")
                        .help("Continue after checks (default false)"))
        )
        .subcommand(
            SubCommand::with_name("no-eth1-sim")
            .about("Runs a simulator that bypasses the eth1 chain.")
                    .arg(Arg::with_name("nodes")
                        .short("n")
                        .long("nodes")
                        .takes_value(true)
                        .default_value("4")
                        .help("Number of beacon nodes"))
                    // .arg(Arg::with_name("validators_per_node")
                    //     .short("v")
                    //     .long("validators_per_node")
                    //     .takes_value(true)
                    //     .default_value("20")
                    //     .help("Number of validators"))
                    // .arg(Arg::with_name("speed_up_factor")
                    //     .short("s")
                    //     .long("speed_up_factor")
                    //     .takes_value(true)
                    //     .default_value("4")
                    //     .help("Speed up factor"))
                    // .arg(Arg::with_name("end_after_checks")
                    //     .short("e")
                    //     .long("end_after_checks")
                    //     .takes_value(false)
                    //     .help("End after checks (default true)"))
        )
}

pub fn run_eth1_sim(matches: &ArgMatches) -> Result<(), String> {

    // speed_up_factor
        // .takes_value(true) -> param指定しなくても、デフォルト値で値が入ってくる＝＞is_present=trueになる。
    let is_present_speed_up_factor = matches.is_present("speed_up_factor");
    println!("speed_up_factor is_present={}", is_present_speed_up_factor);
    let speed_up_factor = value_t!(matches, "speed_up_factor", u64).expect("missing speed_up_factor default");
    println!("spd up = {}", speed_up_factor);
    println!("---");


    // continue_after_checks 
    let is_present_continue_after_checks = matches.is_present("continue_after_checks");
    println!("continue_after_checks is_present={}", is_present_continue_after_checks);

    if is_present_continue_after_checks {
        let continue_after_checks = value_t!(matches, "continue_after_checks", bool).expect("NG!");
        println!("value={}", continue_after_checks);
    } else {
        println!("value is nothing (default)");
    }
    println!("---");

    // //println!("speed_up_factor {}", continue_after_checks);
    // let continue_after_checks = value_t!(matches, "continue_after_checks", bool).expect("NG!");
    // println!("atai = {}", continue_after_checks);


    // let continue_after_checks = matches.is_present("continue_after_checks");
    // let param = value_t!(matches, "continue_after_checks", u64).expect("missing default");
    // println!("atai = {}", param);

    // //println!("speed_up_factor {}", continue_after_checks);

    Ok(())
}

fn main() {

    let matches = cli_app().get_matches();
    match matches.subcommand() {
        ("eth1-sim", Some(matches)) => match run_eth1_sim(matches) {
            Ok(()) => println!("Simulation exited successfully"),
            Err(e) => {
                eprintln!("Simulation exited with error: {}", e);
                std::process::exit(1)
            }
        },
        _ => {
            eprintln!("Invalid subcommand. Use --help to see available options");
            std::process::exit(1)
        }
    }

    println!("Hello, world!");
}
