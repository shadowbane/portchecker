# PortChecker: Get a clear picture of port accessibility

## Usage

```Bash
portchecker HOST PORT [TIMEOUT] [RETRY]
```

## Parameters

```bash
portcheker --help
```

```bash
Parameters:
  HOST                🌐 The IP address or domain name you want to check.
  PORT                🚪 The specific port to probe (1-65535).
  TIMEOUT (optional)  ⏱️ Time in seconds before a request times out (1-15, default: 10).
  RETRY (optional)    🔁 Number of times to retry the test (1-10, default: 3).
```

## Example

```bash
portchecker example.com 80 5 2
```

This command checks port 80 on example.com with a 5-second timeout, retrying twice for reliable results.

