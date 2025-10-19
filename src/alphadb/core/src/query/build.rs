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

use crate::query::column::definecolumn::DefineColumn;

/// Enumeration of supported SQL structure query methods.
///
/// This enum defines the types of table structure operations that can be performed.
/// Each variant corresponds to a different SQL operation with distinct syntax requirements.
#[derive(PartialEq, Eq)]
pub enum StructureQueryMethod {
    /// ALTER TABLE operation for modifying existing table structure
    Altertable,
    /// CREATE TABLE operation for creating new tables
    Createtable,
}

/// A builder for constructing SQL table structure queries.
///
/// `StructureQuery` provides a fluent interface for building SQL queries that modify
/// database table structure. It supports both `CREATE TABLE` and `ALTER TABLE` operations
/// with flexible column definitions, constraints, and options.
///
/// # Fields
///
/// - `method`: The SQL operation type (CREATE TABLE or ALTER TABLE)
/// - `table`: The target table name
/// - `constraints`: Table-level constraints (PRIMARY KEY, FOREIGN KEY, etc.)
/// - `options`: Additional SQL options (ENGINE, CHARSET, etc.)
/// - `definitions`: Column definitions using DefineColumn instances
///
/// # Usage Pattern
///
/// 1. Create a new instance using `createtable()` or `altertable()`
/// 2. Set the table name with `table()`
/// 3. Add column definitions with `definition()`
/// 4. Add constraints with `constraint()`
/// 5. Add options with `options()`
/// 6. Generate the SQL with `build()`
pub struct StructureQuery {
    method: StructureQueryMethod,
    table: Option<String>,
    constraints: Vec<String>,
    options: Vec<String>,
    definitions: Vec<DefineColumn>,
}

impl Default for StructureQuery {
    /// Creates a default `StructureQuery` instance.
    ///
    /// The default configuration uses the `Createtable` method with all other
    /// fields initialized to empty values.
    ///
    /// # Returns
    ///
    /// A new `StructureQuery` instance with default values.
    fn default() -> Self {
        Self {
            method: StructureQueryMethod::Createtable,
            table: None,
            definitions: Vec::new(),
            constraints: Vec::new(),
            options: Vec::new(),
        }
    }
}

impl StructureQuery {
    /// Creates a new `StructureQuery` for CREATE TABLE operations.
    ///
    /// This initializes a query builder specifically for creating new database tables.
    /// The resulting query will use CREATE TABLE syntax with column definitions
    /// wrapped in parentheses.
    ///
    /// # Returns
    ///
    /// A new `StructureQuery` instance configured for CREATE TABLE operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut id_column = DefineColumn::new();
    /// id_column.name("id").datatype("INT").constraint("PRIMARY KEY");
    /// 
    /// let result = query.table("products").definition(id_column.clone()).build();
    /// // Result: "CREATE TABLE products (id INT PRIMARY KEY);"
    /// ```
    pub fn createtable() -> Self {
        Self {
            method: StructureQueryMethod::Createtable,
            ..Default::default()
        }
    }

    /// Creates a new `StructureQuery` for ALTER TABLE operations.
    ///
    /// This initializes a query builder specifically for modifying existing database tables.
    /// The resulting query will use ALTER TABLE syntax with column definitions
    /// not wrapped in parentheses.
    ///
    /// # Returns
    ///
    /// A new `StructureQuery` instance configured for ALTER TABLE operations.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::altertable();
    /// let mut timestamp_column = DefineColumn::new();
    /// timestamp_column.method("ADD COLUMN").name("created_at")
    ///     .datatype("TIMESTAMP").default("CURRENT_TIMESTAMP").default_raw(true);
    /// 
    /// let result = query.table("users").definition(timestamp_column.clone()).build();
    /// // Result: "ALTER TABLE users ADD COLUMN created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP;"
    /// ```
    pub fn altertable() -> Self {
        Self {
            method: StructureQueryMethod::Altertable,
            ..Default::default()
        }
    }

    /// Adds a column definition to the query.
    ///
    /// Column definitions specify the structure and properties of individual columns.
    /// Multiple definitions can be added by calling this method repeatedly.
    /// Each definition is built using the `DefineColumn` builder.
    ///
    /// # Parameters
    ///
    /// - `column_definition`: A `DefineColumn` instance describing the column.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut id_column = DefineColumn::new();
    /// id_column.name("order_id").datatype("INT").constraint("AUTO_INCREMENT");
    /// let mut name_column = DefineColumn::new();
    /// name_column.name("customer_name").datatype("VARCHAR").size("100");
    /// 
    /// let result = query.table("orders")
    ///     .definition(id_column.clone())
    ///     .definition(name_column.clone())
    ///     .build();
    /// // Result: "CREATE TABLE orders (order_id INT AUTO_INCREMENT, customer_name VARCHAR(100));"
    /// ```
    pub fn definition(&mut self, column_definition: DefineColumn) -> &mut Self {
        self.definitions.push(column_definition);
        self
    }

    /// Adds a table-level constraint to the query.
    ///
    /// Table constraints apply to the entire table or multiple columns, such as
    /// primary keys, foreign keys, unique constraints, or check constraints.
    /// Multiple constraints can be added by calling this method repeatedly.
    ///
    /// # Parameters
    ///
    /// - `constraint`: The constraint specification as a string.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut id_column = DefineColumn::new();
    /// id_column.name("user_id").datatype("INT");
    /// let mut email_column = DefineColumn::new();
    /// email_column.name("email").datatype("VARCHAR").size("255");
    /// 
    /// let result = query.table("user_profiles")
    ///     .definition(id_column.clone())
    ///     .definition(email_column.clone())
    ///     .constraint("UNIQUE KEY unique_email (email)")
    ///     .build();
    /// // Result: "CREATE TABLE user_profiles (user_id INT, email VARCHAR(255), UNIQUE KEY unique_email (email));"
    /// ```
    pub fn constraint<S: Into<String>>(&mut self, constraint: S) -> &mut Self {
        self.constraints.push(constraint.into());
        self
    }

    /// Adds SQL options to the query.
    ///
    /// Options are database-specific settings that affect table behavior or storage,
    /// such as storage engine, character set, collation, or other table options.
    /// Multiple options can be added by calling this method repeatedly.
    ///
    /// # Parameters
    ///
    /// - `option`: The SQL option specification as a string.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut id_column = DefineColumn::new();
    /// id_column.name("id").datatype("INT").constraint("AUTO_INCREMENT");
    /// let mut message_column = DefineColumn::new();
    /// message_column.name("message").datatype("TEXT");
    /// 
    /// let result = query.table("logs")
    ///     .definition(id_column.clone())
    ///     .definition(message_column.clone())
    ///     .options("ENGINE=InnoDB")
    ///     .options("DEFAULT CHARSET=utf8mb4")
    ///     .build();
    /// // Result: "CREATE TABLE logs (id INT AUTO_INCREMENT, message TEXT) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;"
    /// ```
    pub fn options<S: Into<String>>(&mut self, option: S) -> &mut Self {
        self.options.push(option.into());
        self
    }

    /// Sets the target table name for the query.
    ///
    /// The table name specifies which database table the operation will affect.
    /// This is required for both CREATE TABLE and ALTER TABLE operations.
    ///
    /// # Parameters
    ///
    /// - `table`: The name of the target table.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut item_column = DefineColumn::new();
    /// item_column.name("item_id").datatype("INT");
    /// 
    /// let mut result = query.table("inventory").definition(item_column.clone()).build();
    /// // Result: "CREATE TABLE inventory (item_id INT);"
    /// ```
    pub fn table<S: Into<String>>(&mut self, table: S) -> &mut Self {
        self.table = Some(table.into());
        self
    }

    /// Generates the final SQL query string.
    ///
    /// This method combines all the configured components into a properly formatted
    /// SQL query. The structure varies based on the query method:
    ///
    /// - **CREATE TABLE**: Column definitions are wrapped in parentheses
    /// - **ALTER TABLE**: Column definitions are not wrapped in parentheses
    ///
    /// The generated query includes the method, table name, column definitions,
    /// constraints, options, and ends with a semicolon.
    ///
    /// # Returns
    ///
    /// A `String` containing the complete SQL query.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alphadb_core::query::build::StructureQuery;
    /// use alphadb_core::query::column::DefineColumn;
    ///
    /// let mut query = StructureQuery::createtable();
    /// let mut session_column = DefineColumn::new();
    /// session_column.name("session_id").datatype("VARCHAR").size("128").constraint("PRIMARY KEY");
    /// let mut user_column = DefineColumn::new();
    /// user_column.name("user_id").datatype("INT").constraint("NOT NULL");
    /// 
    /// let result = query.table("sessions")
    ///     .definition(session_column.clone())
    ///     .definition(user_column.clone())
    ///     .constraint("FOREIGN KEY (user_id) REFERENCES users(id)")
    ///     .options("ENGINE=InnoDB")
    ///     .build();
    /// // Result: "CREATE TABLE sessions (session_id VARCHAR(128) PRIMARY KEY, user_id INT NOT NULL, FOREIGN KEY (user_id) REFERENCES users(id)) ENGINE=InnoDB;"
    /// ```
    pub fn build(&mut self) -> String {
        let mut query: String;

        match self.method {
            StructureQueryMethod::Createtable => query = "CREATE TABLE".to_string(),
            StructureQueryMethod::Altertable => query = "ALTER TABLE".to_string(),
        }

        if let Some(table) = &self.table {
            query = format!("{query} {table}");
        }

        let mut column_definitions = self.definitions.iter().map(ToString::to_string).collect::<Vec<_>>();
        column_definitions.extend(self.constraints.clone());

        // Column definitions for createtable method have to be wrapped in parentheses
        if self.method == StructureQueryMethod::Createtable {
            query = format!("{query} ({})", column_definitions.join(", "));
        }

        // Column definitions for altertable method should not be wrapped in parentheses
        if self.method == StructureQueryMethod::Altertable {
            query = format!("{query} {}", column_definitions.join(", "));
        }

        if !self.options.is_empty() {
            query = format!("{query} {}", self.options.join(" "));
        }

        query.push(';');

        return query.clone();
    }
}
