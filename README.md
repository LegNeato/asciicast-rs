# [asciicast]

[![Build Status](https://travis-ci.org/LegNeato/asciicast-rs.svg?branch=master)](https://travis-ci.org/LegNeato/asciicast-rs)

A Rust library for working with the [Asciicast file format][versions].
Asciicast is used by [Asciinema][asciinema] to play back terminal
recordings.

### Installation

`asciicast` is available on [crates.io](https://crates.io/crates/asciicast) and can be included in your Cargo enabled project like this:

```toml
[dependencies]
asciicast = "0.2.2"
```

Then include it in your code like this:

```rust
extern crate asciicast;
```

### Usage

This library exports [version 2][v2] of the `asciicast` format by default. [Version 1][v1] will eventually be available as a subcrate.

#### Example (v2)

```rust
extern crate asciicast;

let entry = asciicast::Entry {
    time: 1.234,
    event_type: asciicast::EventType::Output,
    event_data: String::new("text data"),
};

println!("{:?}", entry);
```

### License

`asciicast` is licensed under either of the following, at your option:

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

[asciicast]: https://github.com/LegNeato/asciicast-rs
[asciinema]: https://asciinema.org
[v1]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v1.md
[v2]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
[versions]: https://github.com/asciinema/asciinema/tree/develop/doc
