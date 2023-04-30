//#==============================================================================
//# Copyright (c) 2022-2023 Mattsoft/PoiXson
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

pub struct FileFinder {
	found: String,
}

impl FileFinder {

	pub fn new() -> Self {
		Self {
			found: "".to_string(),
		}
	}

	pub fn files(mut self, files: Vec<&str>) -> Self {
		for f in files {
			self = self.file(f.to_string());
		}
		self
	}

	pub fn file(mut self, file: String) -> Self {
		if ! file.is_empty() {
			if self.found.is_empty() {
				if std::path::Path::new(&file).is_file() {
					self.found = file.clone();
				}
			}
		}
		self
	}

	pub fn found(self) -> String {
		self.found.clone()
	}

}
