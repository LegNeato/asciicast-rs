# [asciicast]

A Rust library for working with the [Asciicast][asciicast] file format.
[Asciicast][asciicast] is used by [Asciinema][asciinema] to play back
terminal recordings.

### Installation

`asciicast` is available on [crates.io](https://crates.io/crates/asciicast) and can be included in your Cargo enabled project like this:

```toml
[dependencies]
asciicast = "0.1.0"
```

Then include it in your code like this:

```rust
extern crate asciicast;
```

### Usage

This library exports [version 2][v2] of the `asciicast` format by default. [Version 1][v1] is available as a subcrate as `asciicast::v1`.

#### Example (v2)

```rust
extern crate asciicast;

use asciicast;
use std::str::FromStr;
use std::string::ToString;

let entry = asciicast::Entry {
    time: 1.234,
    event_type: asciicast::EventType::Output,
    event_data: String::new("text data"),
};

println!("{:?}", entry);
```

[asciicast]: https://github.com/Legneato/asciicast
[asciinema]: https://asciinema.org
[v1]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v1.md
[v2]: https://github.com/asciinema/asciinema/blob/develop/doc/asciicast-v2.md
[versions]: https://github.com/asciinema/asciinema/tree/develop/doc
