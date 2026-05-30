# CLI Reference

## Commands

### scan
Compares a template env file against one or more target env files and reports missing, extra, and empty keys.

Example:

```bash
envsentinel scan --root .
```

### diff
Prints the key differences between a template file and one or more target files.

Example:

```bash
envsentinel diff --template .env.example --target .env
```

### validate
Checks env file syntax, duplicate keys, malformed lines, and missing quotes where needed.

Example:

```bash
envsentinel validate --target .env
```

### init
Generates a starter `.env.example` from an existing `.env` file.

Example:

```bash
envsentinel init --root .
```

### watch
Continuously reruns scan and validate in a polling loop until interrupted.

Example:

```bash
envsentinel watch --root .
```
