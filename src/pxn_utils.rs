//# ================================================================================
//# Copyright (c) 2022-2025 Mattsoft/PoiXson
//# <https://mattsoft.net> <https://poixson.com>
//# Released under the AGPL 3.0 + ADD-PXN-V1
//#
//# Description: Common library for PoiXson projects
//#
//# ================================================================================
//#
//# This program is free software: you can redistribute it and/or modify it under
//# the terms of the GNU Affero General Public License + ADD-PXN-V1 as published by
//# the Free Software Foundation, either version 3 of the License, or (at your
//# option) any later version, with the addition of ADD-PXN-V1.
//#
//# This program is distributed in the hope that it will be useful, but WITHOUT ANY
//# WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
//# PARTICULAR PURPOSE.
//#
//# See the GNU Affero General Public License for more details
//# <http://www.gnu.org/licenses/agpl-3.0.en.html> and Addendum ADD-PXN-V1
//# <https://dl.poixson.com/ADD-PXN-V1.txt>
//#
//# **ADD-PXN-V1 - Addendum to the GNU Affero General Public License (AGPL)**
//# This Addendum is an integral part of the GNU Affero General Public License
//# (AGPL) under which this software is licensed. By using, modifying, or
//# distributing this software, you agree to the following additional terms:
//#
//# 1. **Source Code Availability:** Any distribution of the software, including
//#    modified versions, must include the complete corresponding source code. This
//#    includes all modifications made to the original source code.
//# 2. **Free of Charge and Accessible:** The source code and any modifications to
//#    the source code must be made available to all with reasonable access to the
//#    internet, free of charge. No fees may be charged for access to the source
//#    code or for the distribution of the software, whether in its original or
//#    modified form. The source code must be accessible in a manner that allows
//#    users to easily obtain, view, and modify it. This can be achieved by
//#    providing a link to a publicly accessible repository (e.g., GitHub, GitLab)
//#    or by including the source code directly with the distributed software.
//# 3. **Documentation of Changes:** When distributing modified versions of the
//#    software, you must provide clear documentation of the changes made to the
//#    original source code. This documentation should be included with the source
//#    code, and should be easily accessible to users.
//# 4. **No Additional Restrictions:** You may not impose any additional
//#    restrictions on the rights granted by the AGPL or this Addendum. All
//#    recipients of the software must have the same rights to use, modify, and
//#    distribute the software as granted under the AGPL and this Addendum.
//# 5. **Acceptance of Terms:** By using, modifying, or distributing this software,
//#    you acknowledge that you have read, understood, and agree to comply with the
//#    terms of the AGPL and this Addendum.
//# ================================================================================

use log::Level;
use std::io::Write;

use env_logger;



pub fn stdout_flush() {
	std::io::stdout().flush().unwrap();
}



pub fn print_type_of<T>(_: T) {
	println!("{}", std::any::type_name::<T>())
}



pub fn init_logging(verbose: i8, quiet: i8) {
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



/// Makes a safe filename from a path
/// /etc/some.file -> etc-some.file
pub fn safe_file_from_path(file: String) -> String {
	let mut f:String = file.clone();
	if '/' == f.chars().take(1).last().unwrap() {
		f.remove(0);
	}
	f.chars().map(|x|
		match x {
			'A'..='Z' => x,
			'a'..='z' => x,
			'.' | '-' | '_' => x,
			_ => '-',
		}).collect()
}
#[test]
fn test_safe_file_from_path() {
	assert_eq!( safe_file_from_path("/etc/some.file".to_string()), "etc-some.file" );
}



pub fn remove_white_space_lines(source: String) -> String {
	let mut result = String::new();
	let mut first = true;
	'LINES_LOOP:
	for line in source.lines() {
		let str = line.trim();
		if str.is_empty() {
			continue 'LINES_LOOP;
		}
		if first { first = false; } else {
			result.push_str("\n");
		}
		result.push_str( line.trim() );
	}
	result
}
#[test]
fn test_remove_white_space_lines() {
	let str = "\n\nAbc\n\ndef\n\n".to_string();
	assert_eq!( remove_white_space_lines(str), "Abc\ndef") ;
}



pub fn remove_head_comments(source: String) -> (u32, String) {
	let mut result = String::new();
	let mut first = true;
	let mut head = true;
	let mut removed: u32 = 0;
	'LINES_LOOP:
	for line in source.lines() {
		if head {
			if line.is_empty()
			|| line.starts_with("//")
			|| line.starts_with("#")
			|| line.starts_with(";") {
				removed += 1;
				continue 'LINES_LOOP;
			}
			head = false;
		}
		if first { first = false; } else {
			result.push_str("\n");
		}
		result.push_str(line);
	}
	(removed, result)
}
#[test]
fn test_remove_head_comments() {
	let str = "\n\n# This\n// is a\n; comment\n\nAbc\n\ndef\n\n".to_string();
	let (removed, result) = remove_head_comments(str);
	assert_eq!(result, "Abc\n\ndef\n");
	assert_eq!(removed, 6);
}
