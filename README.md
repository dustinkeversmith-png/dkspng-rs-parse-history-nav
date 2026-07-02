# Macro OS Engines App

This package merges the previously separate Rust applications into **one Cargo application** while keeping each engine modularly separated.

## Engine modules

```txt
src/
  parse/       # ambiguity-tolerant inline macro parser
  context/     # context hierarchy, local queues/currents, aliases, symbols
  navigation/  # typed alias/navigation resolver and dry-run plans
  history/     # append-only history events, JSONL store, frequency/suggestion scoring
  watchdog/    # watch specs, rules, routines, simulated file events, action planner
```

The crate exposes a library named `macro_os_engines` and one binary named `macro-os`.

## Validation

```bash
cargo fmt
cargo test
```

## CLI examples

### Parse engine

```bash
cargo run --bin macro-os -- parse examples/ambiguous_macros.txt --pretty
```

### Context engine

```bash
cargo run --bin macro-os -- context index examples/project_contexts.txt
cargo run --bin macro-os -- context tree examples/project_contexts.txt --root project
cargo run --bin macro-os -- context inspect examples/project_contexts.txt parser
```

### Navigation engine

```bash
cargo run --bin macro-os -- nav mock
cargo run --bin macro-os -- nav resolve parser --scope parser
cargo run --bin macro-os -- nav plan parser-workspace --scope project --action open
```

### History engine

```bash
cargo run --bin macro-os -- history print examples/mock_history_events.jsonl
cargo run --bin macro-os -- history stats examples/mock_history_events.jsonl --limit 10
cargo run --bin macro-os -- history suggest examples/mock_history_events.jsonl parser --context parser --workspace macro_processor
cargo run --bin macro-os -- history mock --out .macro/history.jsonl
```

### Watchdog engine

```bash
cargo run --bin macro-os -- watchdog validate examples/watch_spec.json
cargo run --bin macro-os -- watchdog list-rules examples/watch_spec.json
cargo run --bin macro-os -- watchdog simulate examples/watch_spec.json examples/file_events.jsonl
cargo run --bin macro-os -- watchdog simulate examples/watch_spec.json examples/file_events.jsonl --expand-routines
```

## Design note

This intentionally remains one application, not a Cargo workspace. The engines are separated by module boundaries, so later you can split any module back into a crate if it grows too large.


## Added Deep Fixture Tests

This package now includes integration-style tests for the requested engines:

```bash
cargo test context_file_tree_fixture_assigns_unique_context_layers_and_indexes_up_down
cargo test watchdog_filters_file_types_ignores_paths_and_expands_routines
cargo test watchdog_timer_event_runs_timely_routine_from_fixture
cargo test parser_deeply_nested_commands_are_inserted_into_database_and_searchable
cargo test history_log_tracks_file_navigation_explorer_locations_and_console_commands
```

New fixtures live under:

```text
tests/fixtures/deep_tree/
tests/fixtures/deep_nested_macros.md
tests/fixtures/watch_spec_file_types_and_timer.json
tests/fixtures/file_change_events.jsonl
tests/fixtures/history_navigation_commands.jsonl
```

See `docs/TEST_FIXTURE_COVERAGE.md` for details.
