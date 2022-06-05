
use log::Level;

pub mod utils;
pub mod file_finder;



pub fn print_type_of<T>(_: T) {
	println!("{}", std::any::type_name::<T>())
}



pub fn pxn_init_logging(verbose: i8, quiet: i8) {
	let verbosity: i8 = verbose - quiet;
	{
		let lvl = match verbosity {
			// -qq
			-1=> Level::Error.to_level_filter(),
			// -q
			0 => Level::Warn.to_level_filter(),
			// -v
			1 => Level::Info.to_level_filter(),
			// -vv
			2 => Level::Debug.to_level_filter(),
			_ => {
				// -qqq
				if verbosity < -1 {
					log::LevelFilter::Off
				} else
				// -vvv
				if verbosity > 2 {
					Level::Trace.to_level_filter()
				} else {
					Level::Warn.to_level_filter()
				}
			},
		};
		env_logger::Builder::new()
			.filter_level(lvl)
			.init();
	}
	log_panics::init();
}



pub fn get_timestamp() -> String {
	chrono::Local::now()
		.to_rfc2822()
}
