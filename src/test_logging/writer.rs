use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct TestOutputWriter {
    output_dir: PathBuf,
    name_prefix: String,
}

impl TestOutputWriter {
    pub fn new(output_dir: impl Into<PathBuf>, name_prefix: impl Into<String>) -> Result<Self> {
        let output_dir = output_dir.into();
        let name_prefix = name_prefix.into();

        fs::create_dir_all(&output_dir)
            .with_context(|| format!("failed to create output dir {}", output_dir.display()))?;

        Ok(Self {
            output_dir,
            name_prefix,
        })
    }

    pub fn write_json<T: Serialize>(&self, value: &T) -> Result<PathBuf> {
        let path = self.output_dir.join(format!(
            "{}_{}.json",
            sanitize_name(&self.name_prefix),
            now_unix_ms()
        ));

        fs::write(&path, serde_json::to_string_pretty(value)?)?;
        Ok(path)
    }
}

/// Backward-compatible alias for older call sites.
pub type TestLogWriter = TestOutputWriter;

fn now_unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn sanitize_name(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
