use alphadb_core::utils::errors::AlphaDBError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Expr {
    Column { name: String },
    Value { value: Value },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Column { name } => write!(f, "{name}"),
            Expr::Value { value } => match value {
                Value::String(s) => write!(f, "'{}'", s.replace('\'', "''")),
                Value::Null => write!(f, "NULL"),
                _ => write!(f, "{value}"),
            },
        }
    }
}

#[derive(Debug)]
pub enum CompOp {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
}

impl CompOp {
    pub fn as_sql(&self) -> &'static str {
        match self {
            CompOp::Eq => "=",
            CompOp::Ne => "!=",
            CompOp::Lt => "<",
            CompOp::Lte => "<=",
            CompOp::Gt => ">",
            CompOp::Gte => ">=",
        }
    }
}

impl<'de> Deserialize<'de> for CompOp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "=" => Ok(CompOp::Eq),
            "!=" | "<>" => Ok(CompOp::Ne),
            "<" => Ok(CompOp::Lt),
            "<=" => Ok(CompOp::Lte),
            ">" => Ok(CompOp::Gt),
            ">=" => Ok(CompOp::Gte),
            _ => Err(serde::de::Error::unknown_variant(&s, &["=", "!=", "<>", "<", "<=", ">", ">="])),
        }
    }
}

impl Serialize for CompOp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_sql())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Condition {
    Comparison { op: CompOp, left: Expr, right: Expr },
    And { conditions: Vec<Condition> },
    Or { conditions: Vec<Condition> },
    Not { condition: Box<Condition> },
    IsNull { column: String },
    IsNotNull { column: String },
    In { column: String, values: Vec<Value> },
    NotIn { column: String, values: Vec<Value> },
}

fn format_values(values: &[Value]) -> String {
    values
        .iter()
        .map(|v| match v {
            Value::String(s) => format!("'{}'", s.replace('\'', "''")),
            Value::Null => "NULL".to_string(),
            _ => v.to_string(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

impl Condition {
    pub fn to_sql(&self) -> String {
        match self {
            Condition::Comparison { op, left, right } => {
                format!("{left} {} {right}", op.as_sql())
            }
            Condition::And { conditions } => conditions.iter().map(|c| format!("({})", c.to_sql())).collect::<Vec<_>>().join(" AND "),
            Condition::Or { conditions } => conditions.iter().map(|c| format!("({})", c.to_sql())).collect::<Vec<_>>().join(" OR "),
            Condition::Not { condition } => {
                format!("NOT ({})", condition.to_sql())
            }
            Condition::IsNull { column } => format!("{column} IS NULL"),
            Condition::IsNotNull { column } => format!("{column} IS NOT NULL"),
            Condition::In { column, values } => {
                format!("{column} IN ({})", format_values(values))
            }
            Condition::NotIn { column, values } => {
                format!("{column} NOT IN ({})", format_values(values))
            }
        }
    }
}

pub fn condition_to_sql(json: &Value) -> Result<String, AlphaDBError> {
    let condition: Condition = serde_json::from_value(json.clone())?;
    Ok(condition.to_sql())
}
