#![allow(unused)]

use clap::{App, Arg, Parser};
use clap::arg;

mod team;
mod repo;
mod dbscripts;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let mut doteam = false;
    let mut dorepo = false;
    let mut execute = false;
    let mut dbscripts = false;

    let args = App::new("Tenant Boarding")
        .author("DevStudio Team")
        .about("Use this to setup a tenant")
        .args(&[
            Arg::new("doteam")
                .short('t')
                .long("doteam")
                .takes_value(false)
                .help("create github team name"),
            arg!(dorepo: -r --dorepo "create github repo"),
            arg!(dbscripts: -d --dbscripts "create db scripts"),
            arg!(execute: -e --execute "execute for real.  without it will just be a dry run")
        ]).get_matches();

    //set the flags you need
    if args.is_present("doteam") {
        doteam = true;
        println!("{}", doteam)
    }

    if args.is_present("dorepo") {
        dorepo = true;
        println!("{}", dorepo)
    }

    if args.is_present("dbscripts") {
        dbscripts = true;
        println!("{}", dbscripts)
    }

    if args.is_present("execute") {
        execute = true;
        println!("{}", execute)
    }

    //now call each function corresponding to the flags
    //remember that passing in EXECUTE will control if that actually runs
    team::do_team(execute);
    //repo::do_repo(execute);
    //dbscripts::dbscripts(execute);


    /*let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

}
