/// Basic logging setup to log to the console with `fern`.
pub fn setup() {
    use fern::colors::{Color, ColoredLevelConfig};
    let colors = ColoredLevelConfig::default()
        .info(Color::Green)
        .debug(Color::BrightMagenta)
        .trace(Color::BrightBlue);

    // This sets up a `fern` logger and initializes `log`.
    fern::Dispatch::new()
        // Formats logs
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{:<5}][{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                colors.color(record.level()),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        // Filter out unnecessary stuff
        .level_for("gfx", log::LevelFilter::Off)
        // .level_for("walk", log::LevelFilter::Warn)
        // Set levels for stuff we care about
        .level_for("threething", log::LevelFilter::Trace)
        .level_for("gfx_device_gl", log::LevelFilter::Off)
        // Hooks up console output.
        // env var for outputting to a file?
        // Haven't needed it yet!
        .chain(std::io::stdout())
        .apply()
        .expect("Could not init logging!");
}
