//#==============================================================================
//# Copyright (c) 2022 Mattsoft/PoiXson
//# <https://mattsoft.net> <https://poixson.com>
//# Released under the AGPL 3.0
//#
//# Description: Common library for PoiXson projects
//#
//# This program is free software: you can redistribute it and/or modify
//# it under the terms of the GNU Affero General Public License as
//# published by the Free Software Foundation, either version 3 of the
//# License, or (at your option) any later version.
//#
//# This program is distributed in the hope that it will be useful,
//# but WITHOUT ANY WARRANTY; without even the implied warranty of
//# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//# GNU Affero General Public License for more details.
//#
//# You should have received a copy of the GNU Affero General Public License
//# along with this program.  If not, see <https://www.gnu.org/licenses/>.
//#==============================================================================

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
