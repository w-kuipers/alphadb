// Copyright (C) 2024 Wibo Kuipers
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs;
use std::path::Path;

use crate::core::utils::errors::AlphaDBError;
use crate::verification::VersionTrace;
use crate::version_source::build_version_source_from_dir;

/// Resolve a version source argument into its JSON contents.
///
/// `input` can be a path to a directory of version source files, a path to a
/// single version source file, or the JSON contents themselves.
pub fn parse_version_source(input: &str) -> Result<String, AlphaDBError> {
    let path = Path::new(input);

    if path.is_dir() {
        let value = build_version_source_from_dir(&path.to_path_buf())?;
        return Ok(serde_json::to_string(&value)?);
    }

    if path.is_file() {
        return fs::read_to_string(path).map_err(|e| AlphaDBError {
            message: format!("An error occured while opening the version source at '{}': {}", path.display(), e),
            error: "version-source-read-failed".to_string(),
            version_trace: VersionTrace::new(),
        });
    }

    Ok(input.to_string())
}
