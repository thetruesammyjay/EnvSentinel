# EnvSentinel

EnvSentinel is a Rust CLI for checking `.env` files against a template, validating file syntax, generating starter templates, and watching for drift over time.

## What It Does

- Scans projects for common env files such as `.env`, `.env.example`, and `.env.local`.
- Compares a template file against one or more target files.
- Validates env file syntax, duplicate keys, malformed lines, and missing quotes.
- Generates a starter `.env.example` from an existing `.env`.
- Watches for changes and reruns the same scan and validate core.

## Commands

- `envsentinel scan` - compare a template against discovered or explicit targets.
- `envsentinel diff` - print key-level drift between a template and target files.
- `envsentinel validate` - check env file syntax and content issues.
- `envsentinel init` - generate `.env.example` from `.env`.
- `envsentinel watch` - continuously rerun the scan and validate cycle.

See [docs/cli.md](docs/cli.md) for the command reference.

## Installation

Build from source:

```bash
cargo build --release
```

Run locally:

```bash
cargo run -- scan
```

## Usage Examples

```bash
envsentinel scan --root .
envsentinel diff --template .env.example --target .env
envsentinel validate --target .env
envsentinel init --root .
envsentinel watch --root .
```

## Configuration

EnvSentinel reads `config/envsentinel.toml` by default.

```toml
[defaults]
strict = false
template_file = ".env.example"
target_files = [".env", ".env.local"]
ignore_directories = ["node_modules", "target", "dist"]
```

Unknown keys and invalid types are treated as config errors. See [docs/config.md](docs/config.md).

## Output Formats

Use `--json` for machine-readable output and `--markdown` for report-style output. See [docs/output-formats.md](docs/output-formats.md).

## Exit Codes

- `0` - success, no findings.
- `1` - findings were detected, such as missing keys or validation problems.
- `2` - usage or configuration error.

## Development

```bash
cargo test
cargo test --test watch
```

## License

Released under the MIT License. See [LICENSE](LICENSE).


