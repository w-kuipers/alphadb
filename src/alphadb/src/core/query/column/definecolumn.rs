// Copyright (C) 2024 Wibo Kuipers
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

use core::fmt;

/// A builder for creating SQL column definitions.
///
/// `DefineColumn` provides a fluent interface for constructing column definitions
/// that can be used in various SQL operations such as table creation, alteration,
/// and modification.
#[derive(Clone, PartialEq, Eq)]
pub struct DefineColumn {
    name: String,
    contstraints: Vec<String>,
    default: String,
    default_raw: bool,
    column_type: String,
    size: String,
    method: Option<String>,
}

impl Default for DefineColumn {
    fn default() -> Self {
        Self::new()
    }
}

impl DefineColumn {
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    /// let column = DefineColumn::new();
    /// // All fields are empty and ready for configuration
    /// ```
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            column_type: "".to_string(),
            size: "".to_string(),
            contstraints: Vec::new(),
            default: "".to_string(),
            default_raw: false,
            method: None,
        }
    }

    /// Sets the column name.
    ///
    /// The column name appears at the beginning of the column definition
    /// (after any method prefix) and identifies the column in the database schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("user_id")
    ///     .to_sql();
    /// // Result: "user_id"
    /// ```
    pub fn name<S: Into<String>>(&mut self, name: S) -> &mut Self {
        self.name = name.into();
        self
    }

    /// Sets the SQL method for the column definition.
    ///
    /// This is typically used for ALTER TABLE operations where you need to specify
    /// whether you're adding, modifying, or dropping a column. The method appears
    /// at the very beginning of the generated SQL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .method("ADD COLUMN")
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .to_sql();
    /// // Result: "ADD COLUMN email VARCHAR(255)"
    /// ```
    pub fn method<S: Into<String>>(&mut self, method: S) -> &mut Self {
        self.method = Some(method.into());
        self
    }

    /// Sets the SQL data type for the column.
    ///
    /// The type will be automatically converted to uppercase in the final SQL.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("description")
    ///     .datatype("varchar")  // Will be converted to uppercase
    ///     .to_sql();
    /// // Result: "description VARCHAR"
    /// ```
    pub fn datatype<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.column_type = value.into();
        self
    }

    /// Sets the size or length specification for the column data type.
    ///
    /// Many SQL data types accept size parameters, such as VARCHAR(255),
    /// DECIMAL(10,2), or CHAR(1).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("title")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .to_sql();
    /// // Result: "title VARCHAR(255)"
    /// ```
    pub fn size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.size = value.into();
        self
    }

    /// Adds a constraint to the column definition.
    ///
    /// Multiple constraints can be added by calling this method multiple times.
    /// Each constraint will be appended to the column definition and converted
    /// to uppercase.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .constraint("NOT NULL")
    ///     .constraint("UNIQUE")
    ///     .to_sql();
    /// // Result: "email VARCHAR(255) NOT NULL UNIQUE"
    /// ```
    pub fn constraint<S: Into<String>>(&mut self, constraint: S) -> &mut Self {
        self.contstraints.push(constraint.into());
        self
    }

    /// Sets the default value for the column.
    ///
    /// By default, the value will be wrapped in single quotes
    /// unless `default_raw(true)` is called.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("status")
    ///     .datatype("VARCHAR")
    ///     .size("20")
    ///     .default("active")
    ///     .to_sql();
    /// // Result: "status VARCHAR(20) DEFAULT 'active'"
    /// ```
    pub fn default<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.default = value.into();
        self
    }

    /// Controls whether the default value should be wrapped in quotes.
    ///
    /// By default, default values are wrapped in single quotes ('value').
    /// Setting this to `true` will output the raw value without quotes,
    /// which is useful for numeric defaults, function calls, or SQL keywords.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("count")
    ///     .datatype("INT")
    ///     .default("0")
    ///     .default_raw(true)
    ///     .to_sql();
    /// // Result: "count INT DEFAULT 0"
    /// ```
    pub fn default_raw(&mut self, value: bool) -> &mut Self {
        self.default_raw = value;
        self
    }

    /// Generates the SQL column definition string.
    ///
    /// The components are assembled in the following order:
    ///
    /// 1. Method (if specified, e.g., "ADD COLUMN")
    /// 2. Column name
    /// 3. Data type (converted to uppercase)
    /// 4. Size specification (if provided, wrapped in parentheses)
    /// 5. Constraints (each converted to uppercase)
    /// 6. Default value (quoted unless raw is specified)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb::core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .method("ADD COLUMN")
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .constraint("NOT NULL")
    ///     .default("user@example.com")
    ///     .to_sql();
    /// // Result: "ADD COLUMN email VARCHAR(255) NOT NULL DEFAULT 'user@example.com'"
    /// ```
    pub fn to_sql(&self) -> String {
        let mut query = self.name.to_string();

        if !self.column_type.is_empty() {
            query = format!("{query} {}", self.column_type.to_uppercase());
        }

        if let Some(method) = &self.method {
            query = format!("{method} {query}");
        }

        if !self.size.is_empty() {
            query = format!("{query}({})", self.size);
        }

        for attr in &self.contstraints {
            query = format!("{query} {}", attr.to_uppercase());
        }

        if !self.default.is_empty() {
            if self.default_raw {
                query = format!("{query} DEFAULT {}", self.default);
            } else {
                query = format!("{query} DEFAULT '{}'", self.default);
            }
        }

        query.clone()
    }
}

impl fmt::Display for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_sql().fmt(f)
    }
}

impl fmt::Debug for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DefineColumn").field(&self.to_sql()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::DefineColumn;

    #[test]
    fn builds_full_column_definition() {
        let sql = DefineColumn::new()
            .method("ADD COLUMN")
            .name("email")
            .datatype("varchar")
            .size("255")
            .constraint("not null")
            .constraint("unique")
            .default("user@example.com")
            .to_sql();

        assert_eq!(sql, "ADD COLUMN email VARCHAR(255) NOT NULL UNIQUE DEFAULT 'user@example.com'");
    }

    #[test]
    fn uses_raw_default_and_formats_as_sql() {
        let mut column = DefineColumn::new();
        column.name("created_at").datatype("timestamp").default("CURRENT_TIMESTAMP").default_raw(true);

        assert_eq!(column.to_sql(), "created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP");
        assert_eq!(column.to_string(), column.to_sql());
        assert_eq!(format!("{column:?}"), "DefineColumn(\"created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP\")");
    }
}
