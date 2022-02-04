//mod github;
use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn do_team(execute: bool) {
    error!("do_team Goes to stderr and file");
    warn!("do_team Goes to stderr and file");
    info!("do_team Goes to stderr and file");
    debug!("do_team Goes to file only");
    trace!("do_team Goes to file only");

    println!("doTeam dry");

    if (execute) {
        println!("executing doTeam");
    }
}