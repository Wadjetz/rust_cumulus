use uuid::Uuid;
use chrono::NaiveDateTime;
use serde_json::Value;

use source_type::SourceType;

#[derive(Debug)]
pub struct Source {
    pub uuid: Uuid,
    pub source_type: SourceType,
    pub data: Value,
    pub error: Option<String>,
    pub created: NaiveDateTime,
    pub updated: NaiveDateTime,
}
