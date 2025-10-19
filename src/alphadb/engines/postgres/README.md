# AlphaDB MySQL Engine

This is the PostgreSQL engine implementation for AlphaDB. It provides Postgres-specific functionality for database operations.

## Usage

The Postgres engine allows you to set connection parameters either at creation time or after creation:

### Creating with connection parameters

```rust
use alphadb_mysql::PostgresEngine;

let engine = PostgresEngine::with_credentials(
    "localhost".to_string(),
    "root".to_string(),
    "password".to_string(),
    "mydatabase".to_string(),
    5432,
);

let mut alphadb = AlphaDB::with_engine(engine);
alphadb.engine.connect()?;
```

### Setting connection parameters after creation

```rust
use alphadb_mysql::MySQLEngine;

let mut engine = MySQLEngine::new();
engine.set_credentials(
    "localhost".to_string(),
    "root".to_string(),
    "password".to_string(),
    "mydatabase".to_string(),
    5432,
);

let mut alphadb = AlphaDB::with_engine(engine);
alphadb.engine.connect()?;
```

## Design

The PostgreSQL engine implements the `AlphaDBEngine` trait and stores connection parameters internally. When `connect()` is called, it uses these stored parameters to establish the database connection. This approach allows the engine to decide what parameters it needs while maintaining a consistent trait interface.

## Error Handling

The engine provides proper error conversion from `PostgresEngineError` to `AlphaDBError` for seamless integration with the main AlphaDB library.
