use crate::test_logging::model::{
    CommandRef, DbRef, FileRef, ParseFileRef, SearchHitRef, SearchRef,
};
use serde_json::{Map, Value};

pub fn run_ref(test_name: &str) -> String {
    format!("run:{}", sanitize_ref_segment(test_name))
}

pub fn file_ref(index: usize) -> FileRef {
    format!("file:{:04}", index + 1)
}

pub fn parse_file_ref(index: usize) -> ParseFileRef {
    format!("parse_file:{:04}", index + 1)
}

pub fn command_ref(source_name: &str, command_id: &str) -> CommandRef {
    format!(
        "command:{}:{}",
        sanitize_ref_segment(source_name),
        sanitize_ref_segment(command_id)
    )
}

pub fn table_ref(table_name: &str) -> String {
    format!("table:{}", table_name)
}

pub fn search_ref(query: &str) -> SearchRef {
    format!("search:{}", query_slug(query))
}

pub fn search_hit_ref(query: &str, hit_index: usize) -> SearchHitRef {
    format!("search_hit:{}:{:04}", query_slug(query), hit_index + 1)
}

pub fn db_ref_for_row(table_name: &str, row: &Map<String, Value>) -> Option<DbRef> {
    match table_name {
        "sources" => json_i64(row, "id").map(|id| format!("db:sources:{id}")),
        "parsed_commands" => json_i64(row, "id").map(|id| format!("db:parsed_commands:{id}")),
        "command_parameters" => {
            let command_db_id = json_i64(row, "command_db_id")?;
            let position = json_i64(row, "position")?;
            Some(format!("db:command_parameters:{command_db_id}:{position}"))
        }
        "command_tags" => {
            let command_db_id = json_i64(row, "command_db_id")?;
            let tag = json_str(row, "tag")?;
            Some(format!("db:command_tags:{command_db_id}:{tag}"))
        }
        "command_references" => {
            let command_db_id = json_i64(row, "command_db_id")?;
            let reference = json_str(row, "reference")?;
            Some(format!("db:command_references:{command_db_id}:{reference}"))
        }
        "command_statuses" => {
            let command_db_id = json_i64(row, "command_db_id")?;
            let status = json_str(row, "status")?;
            Some(format!("db:command_statuses:{command_db_id}:{status}"))
        }
        _ => None,
    }
}

pub fn query_slug(query: &str) -> String {
    query
        .trim()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}

fn sanitize_ref_segment(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn json_i64(row: &Map<String, Value>, key: &str) -> Option<i64> {
    row.get(key).and_then(|value| match value {
        Value::Number(number) => number.as_i64(),
        Value::String(text) => text.parse().ok(),
        _ => None,
    })
}

fn json_str<'a>(row: &'a Map<String, Value>, key: &str) -> Option<&'a str> {
    row.get(key).and_then(|value| value.as_str())
}
