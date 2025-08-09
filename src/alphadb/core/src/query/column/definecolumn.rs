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
/// and modification. The builder pattern allows for flexible and readable
/// column specification.
///
/// # Fields
///
/// - `name`: The column name
/// - `contstraints`: A vector of column constraints (NOT NULL, UNIQUE, etc.)
/// - `default`: The default value for the column
/// - `default_raw`: Whether the default value should be used without quotes
/// - `column_type`: The SQL data type (VARCHAR, INT, etc.)
/// - `size`: The size or length specification for the data type
/// - `method`: Optional SQL method prefix (ADD COLUMN, MODIFY COLUMN, etc.)
///
/// # Thread Safety
///
/// This struct implements `Clone`, `PartialEq`, and `Eq`, making it safe to use
/// across multiple contexts and comparisons.
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

impl DefineColumn {
    /// Creates a new `DefineColumn` instance with default values.
    ///
    /// # Returns
    ///
    /// A new `DefineColumn` instance with all fields set to their default values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
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
    /// # Parameters
    ///
    /// - `name`: The name of the column. Can be any type that implements `Into<String>`.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("user_id")
    ///     .to_string();
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
    /// # Parameters
    ///
    /// - `method`: The SQL method (e.g., "ADD COLUMN", "MODIFY COLUMN", "DROP COLUMN").
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .method("ADD COLUMN")
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .to_string();
    /// // Result: "ADD COLUMN email VARCHAR(255)"
    /// ```
    pub fn method<S: Into<String>>(&mut self, method: S) -> &mut Self {
        self.method = Some(method.into());
        self
    }

    /// Sets the SQL data type for the column.
    ///
    /// The data type determines what kind of data the column can store.
    /// Common types include VARCHAR, INT, DECIMAL, DATE, etc. The type
    /// will be automatically converted to uppercase in the final SQL.
    ///
    /// # Parameters
    ///
    /// - `value`: The SQL data type (case-insensitive).
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("description")
    ///     .datatype("varchar")  // Will be converted to uppercase
    ///     .to_string();
    /// // Result: "description VARCHAR"
    /// ```
    pub fn datatype<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.column_type = value.into();
        self
    }

    /// Sets the size or length specification for the column data type.
    ///
    /// Many SQL data types accept size parameters, such as VARCHAR(255),
    /// DECIMAL(10,2), or CHAR(1). This method sets that size specification.
    ///
    /// # Parameters
    ///
    /// - `value`: The size specification (e.g., "255", "10,2").
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("title")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .to_string();
    /// // Result: "title VARCHAR(255)"
    /// ```
    pub fn size<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.size = value.into();
        self
    }

    /// Adds a constraint to the column definition.
    ///
    /// Constraints specify rules and properties for the column data.
    /// Multiple constraints can be added by calling this method multiple times.
    /// Each constraint will be appended to the column definition and converted
    /// to uppercase.
    ///
    /// # Parameters
    ///
    /// - `constraint`: The constraint to add (e.g., "NOT NULL", "UNIQUE", "AUTO_INCREMENT").
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .constraint("NOT NULL")
    ///     .constraint("UNIQUE")
    ///     .to_string();
    /// // Result: "email VARCHAR(255) NOT NULL UNIQUE"
    /// ```
    pub fn constraint<S: Into<String>>(&mut self, constraint: S) -> &mut Self {
        self.contstraints.push(constraint.into());
        self
    }

    /// Sets the default value for the column.
    ///
    /// The default value will be used when no value is explicitly provided
    /// during insertion. By default, the value will be wrapped in single quotes
    /// unless `default_raw(true)` is called.
    ///
    /// # Parameters
    ///
    /// - `value`: The default value for the column.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("status")
    ///     .datatype("VARCHAR")
    ///     .size("20")
    ///     .default("active")
    ///     .to_string();
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
    /// # Parameters
    ///
    /// - `value`: `true` to use raw value without quotes, `false` to wrap in quotes.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .name("count")
    ///     .datatype("INT")
    ///     .default("0")
    ///     .default_raw(true)
    ///     .to_string();
    /// // Result: "count INT DEFAULT 0"
    /// ```
    pub fn default_raw(&mut self, value: bool) -> &mut Self {
        self.default_raw = value;
        self
    }

    /// Generates the SQL column definition string.
    ///
    /// This method combines all the configured properties into a properly
    /// formatted SQL column definition. The components are assembled in
    /// the following order:
    ///
    /// 1. Method (if specified, e.g., "ADD COLUMN")
    /// 2. Column name
    /// 3. Data type (converted to uppercase)
    /// 4. Size specification (if provided, wrapped in parentheses)
    /// 5. Constraints (each converted to uppercase)
    /// 6. Default value (quoted unless raw is specified)
    ///
    /// # Returns
    ///
    /// A `String` containing the complete SQL column definition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let column = DefineColumn::new()
    ///     .method("ADD COLUMN")
    ///     .name("email")
    ///     .datatype("VARCHAR")
    ///     .size("255")
    ///     .constraint("NOT NULL")
    ///     .default("user@example.com")
    ///     .to_string();
    /// // Result: "ADD COLUMN email VARCHAR(255) NOT NULL DEFAULT 'user@example.com'"
    /// ```
    pub fn to_string(&self) -> String {
        let mut query = format!("{}", self.name);

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

        return query.clone();
    }
}

impl fmt::Display for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_string().fmt(f)
    }
}

impl fmt::Debug for DefineColumn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DefineColumn").field(&self.to_string()).finish()
    }
}
