use crate::cli::Cli;
use fern::{
    Dispatch,
    colors::{Color, ColoredLevelConfig},
};
use log::error;
use std::time::SystemTime;

pub fn setup_logger(args: &Cli) -> Result<(), fern::InitError> {
    if !args.log_path.exists()
        && let Err(why) = std::fs::create_dir(args.log_path.clone())
    {
        error!("Failed to create log directory: {}", why);
        return Err(why.into());
    }

    let file_name = format!(
        "{}.log",
        humantime::format_rfc3339_seconds(SystemTime::now())
    );
    let log_path = args.log_path.join(file_name);
    let verbose = args.verbose;
    let log_level = if verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    let colors = ColoredLevelConfig::new()
        .info(Color::Blue)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    Dispatch::new()
        .format(move |out, message, record| {
            let time_str = chrono::Local::now().format("%H:%M:%S").to_string();

            let target = if verbose { record.target() } else { "RIPGUARD" };

            const BLUE: &str = "\x1b[34m";
            const YELLOW: &str = "\x1b[33m";
            const GRAY: &str = "\x1b[90m";
            const RESET: &str = "\x1b[0m";

            out.finish(format_args!(
                "{BLUE}[{YELLOW}{time_str} {} {GRAY}{target}{BLUE}] {RESET}{message}",
                colors.color(record.level()),
            ))
        })
        .level(log_level)
        .level_for("tracing", log::LevelFilter::Off)
        .chain(std::io::stdout())
        .chain(fern::log_file(log_path)?)
        .apply()?;
    Ok(())
}
