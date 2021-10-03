# scheduled-discord-message

This project serves as a starting poing for an app, which can send Discord messages to a Webhook endpoint.

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
cargo run -- -c ./tasks.json
```


## Motivation

> Why this is written in Rust?

This application is intended to run on a Raspberry Pi, where I want to use the CPU as efficiently as possible.
The hypothesis is that Rust allows that, but the benchmark is to be implemented.

## Related links

- https://github.com/halfzebra/tokio-cron-async
- https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050