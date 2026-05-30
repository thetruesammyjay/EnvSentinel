# Configuration

EnvSentinel reads an optional TOML config file from `config/envsentinel.toml` by default.

## Schema

```toml
[defaults]
strict = false
template_file = ".env.example"
target_files = [".env", ".env.local"]
ignore_directories = ["node_modules", "target", "dist"]
```

## Supported keys

- `defaults.strict`
- `defaults.template_file`
- `defaults.target_files`
- `defaults.ignore_directories`

Unknown keys and invalid types are reported as config errors.
