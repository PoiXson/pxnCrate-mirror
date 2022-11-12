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

use tempfile::NamedTempFile;



/// Creates a new temp file /tmp/tmp.xxxxxxxxxx
pub fn new_temp_file() -> (String, NamedTempFile) {
	let tmp: NamedTempFile =
		tempfile::Builder::new()
			.prefix("tmp.")
			.rand_bytes(10)
			.tempfile().unwrap();
	( tmp.path().display().to_string(), tmp )
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
