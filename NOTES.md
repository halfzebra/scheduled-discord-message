# Notes

This document contains notes from the development process.

## Discord Webhook

- https://birdie0.github.io/discord-webhooks-guide/index.html

## Graceful shutdown

In this project I've decided to experiment with graceful shutdown of an app by relying on signal listener from Tokio.

- https://rust-lang.github.io/async-book/06_multiple_futures/02_join.html
- https://rust-lang.github.io/async-book/06_multiple_futures/03_select.html
- https://github.com/mvniekerk/tokio-cron-scheduler/issues/2
- https://github.com/thoo0224/webhook-rs
- https://docs.serde.rs/serde_json


## Building for Raspberry Pi

This CLI runs on a Raspberry Pi, so I needed some guidance on building it for ARM:

- https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

## Reducing binary size

The default config produced a 4MB binary, which is a bit too much.

- https://github.com/johnthagen/min-sized-rust