# scheduled-discord-message

This repository , which can send Discord messages to a Webhook endpoint.

## Usage

Create a file `tasks.json`:

```json
[
  {
    "url": "<discord-webhook-url>",
    "schedule": "0 37 13 * * *",
    "webhook": {
      "username": "Sally",
      "content": "Time for the daily Sally!"
    }
  }
]
```

Then run:

```bash
cargo build --relase

# Don't forget to update the URL
./target/release/scheduled-discord-message -c ./examples/tasks.json
```

## Motivation

> Why this is written in Rust?

This application is intended to run on a Raspberry Pi, where I want to use the CPU as efficiently as possible.
The hypothesis is that Rust allows that, but the benchmark is to be implemented.

## Related links

- https://github.com/halfzebra/tokio-cron-async
