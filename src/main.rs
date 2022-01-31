#![allow(unused)]

use clap::{App, Arg, Parser};
use clap::arg;

mod mods;

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
    let args = App::new("Tenant Boarding")
        .author("DevStudio Team")
        .about("Use this to setup a tenant")
        .args(&[
            Arg::new("team")
                .short('t')
                .long("doteam")
                .takes_value(false)
                .help("create github team name"),
            arg!(-r --dorepo "create github repo"),
            arg!(-d --dbscripts "create db scripts"),
            arg!(-e --execute "execute for real.  without it will just be a dry run")
        ]).get_matches();

    if args.is_present("team") {
        println!("i see it!")
    }

    /*let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }*/

}
