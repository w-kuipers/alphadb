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

/// ** Allowed verification issue level**
///
/// Matches VerificationIssueLevel, but adds an additional value: All.
/// Used for functions where VerificationIssueLevel is decided by the user.
///
/// Low: Will pass with verification errors below level high.
/// High: Will pass with verification errors below level Critical.
/// Critical: Will ignore all errors.
/// All: Will fail with an error of any level.
#[derive(Debug, Copy, Clone)]
pub enum ToleratedVerificationIssueLevel {
    /// Low: Will pass with verification errors below level high.
    Low,
    /// High: Will pass with verification errors below level Critical.
    High,
    /// Critical: Will ignore all errors.
    Critical,
    /// All: Will fail with an error of any level.
    All,
}
