// Copyright (C) 2025 Wibo Kuipers
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

use serde_json::Value;

use crate::{
    utils::consolidate::column::get_column_type,
    verification::issue::{VerificationIssue, VerificationIssueLevel, VersionTrace},
};

/// Compatibility rule for type-attribute incompatibilities
pub struct ColumnCompatibilityRule {
    pub incompatible: &'static [&'static str],
    pub attribute: &'static str,
}

/// Helper function to check type incompatibilities with specific attributes
pub fn verify_column_type_compatibility(
    issues: &mut Vec<VerificationIssue>,
    checking_type: &str,
    rules: &[ColumnCompatibilityRule],
    column_keys: &[&String],
    version_trace: &VersionTrace,
) {
    for rule in rules {
        if !check_column_type_compatibility(checking_type, rule, column_keys) {
            issues.push(VerificationIssue {
                level: VerificationIssueLevel::Critical,
                message: format!("Column type {} is incompatible with attribute {}", checking_type, rule.attribute.to_uppercase()),
                version_trace: version_trace.clone(),
            });
        }
    }
}

pub fn column_contains_type(version_list: &Vec<Value>, column_name: &str, table_name: &str, version: u32) -> bool {
    return match get_column_type(version_list, column_name, table_name, version) {
        Ok(column_type) => column_type.is_some(),
        // If the function returns an error, it has likely already been adressed earlier in the
        // verification process, this function should not create additional issues as they will be
        // solved by solving earlier ones.
        Err(_) => true,
    };
}

pub fn check_column_type_compatibility(checking_type: &str, rule: &ColumnCompatibilityRule, column_keys: &[&String]) -> bool {
    if rule.incompatible.contains(&checking_type.to_lowercase().as_str()) && column_keys.contains(&&String::from(rule.attribute)) {
        return false;
    }

    return true;
}

pub fn check_column_attributes_compatibility<'a>(rule: &'a ColumnCompatibilityRule, column_keys: &'a [&String]) -> Result<(), Vec<&'a str>> {
    let mut incompatible_keys: Vec<&str> = Vec::new();
    if column_keys.contains(&&rule.attribute.to_string()) {
        for incompatible in rule.incompatible {
            if column_keys.contains(&&incompatible.to_string()) {
                incompatible_keys.push(incompatible);
            }
        }
    }

    if !incompatible_keys.is_empty() {
        return Err(incompatible_keys);
    }
    return Ok(());
}
