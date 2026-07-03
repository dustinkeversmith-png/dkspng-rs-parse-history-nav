Implemented the cross-referenced test output dump system per your spec. Components stay mostly unchanged; the test harness still collects artifacts and the new builder layer connects them.

## What was added

### `src/test_logging/`

|Module|Role|
|---|---|
|`model.rs`|`TestOutputDocument`, `TestOutputSchema`, `TestRunInfo`, cross-ref section types, indexes, links, diagnostics, summary|
|`refs.rs`|Stable ref generation (`file:0001`, `command:{source}:{cmd_id}`, `db:{table}:{id}`, etc.)|
|`builder.rs`|`TestOutputBuilder` — assigns refs, builds indexes/links/diagnostics/summary|
|`writer.rs`|`TestOutputWriter` (replaces `TestLogWriter`; alias kept for compatibility)|

### Output shape

The test now writes:

{

"schema": { "name": "macro_os_test_output", "version": "0.1.0", "format": "cross_referenced_test_dump" },

"run": { "run_ref": "run:...", ... },

"sections": { "walk", "parse", "database", "searches" },

"indexes": { "files_by_source_name", "commands_by_db_ref", ... },

"links": [ { "from", "to", "relation" } ],

"diagnostics": [],

"summary": { "file_count", "parsed_command_count", ... }

}

### Cross-references wired

- Walk → `file_ref` on each walked file
- Parse files → `parse_file_ref`, `file_ref`, `command_refs[]`
- Parse commands → `command_ref`, `source_span`, `db_refs[]`
- Database rows → synthetic `_db_ref` on each dumped row
- Search hits → `search_ref`, `search_hit_ref`, resolved `command_ref` / `db_ref`
- Links → `parsed_as`, `produced_command`, `inserted_as`, `produced_hit`, `matched_database_row`, `matched_command`
- Diagnostics → missing parse files, count mismatches, unresolved refs, truncated dumps

### Test update

`tests/parse/real_path_tree_database_test.rs` now uses:

let mut output = TestOutputBuilder::new(TestRunInfo { ... });

output.add_walk_summary(...);

output.add_walk_records(...);

// ... other sections ...

let document = output.build();

TestOutputWriter::new(log_dir, "logged_real_path_walk_macropipeline_database_efficacy")?

.write_json(&document)?;

Also fixed dump truncation: uses `inserted_ids.len().max(100)` so all 148 commands get `_db_ref` links (the earlier 100-row cap caused false `parsed_command_missing_db_ref` warnings).

### Verification

- Unit test `builder_assigns_cross_references` passes
- Integration test `cargo test --test parse` passes
- Latest output: 148 commands, 0 diagnostics, full cross-ref graph

Latest artifact: `C:\Users\Cutie Magic 500\Desktop\desktop_temp_docs\log\logged_real_path_walk_macropipeline_database_efficacy_1782984521274.json`