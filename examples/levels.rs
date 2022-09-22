#[macro_use]
extern crate log;
use rust_logger;

fn main()
{
    let mut rust_logger = rust_logger::builder();
    rust_logger.filter(None, log::LevelFilter::Warn);
    rust_logger.init();
    error!("error message");
    error!("error with fmt: {}", 42);
    warn!("warn message");

    /*
     * NOTE:
     *
     * The following log lines will not be printed, because the filter
     * is set to LevelFilter::Warn
     */
    info!("info message");
    debug!("debug message");
    trace!("trace message");
}
