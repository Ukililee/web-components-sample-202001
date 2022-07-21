# Stock Ticker Service

### What is it?

An HTTP/TCP stock querying service where users can query or subscribe to 1 or more stock tickers.
Users have credits that they spend on each tick (every second) that they are subscribed for, 1 credit equals 1 Stock Subscription for 1 Second.
Authentication is not done, also stock values are randomly generated.

Main point of this repo is showcase of using Actix with Actor Models to build Rust Concurrent Servers.

# Instructions

```shell
$ cargo build
$ cargo run
``