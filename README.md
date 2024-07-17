# afteroff

[![crates.io](https://img.shields.io/crates/v/afteroff.svg)](https://crates.io/crates/afteroff)
[![Docs.rs](https://docs.rs/afteroff/badge.svg)](https://docs.rs/afteroff)
![License](https://img.shields.io/badge/license-MIT-blue)


`afteroff` is a lightweight, efficient utility designed to monitor a specific process on your Linux system and power off the machine when the process terminates. It is written in Rust to ensure high performance and low resource usage.

## Features

- **Efficient Process Monitoring**: Uses direct system calls to check process status, minimizing CPU usage.
- **Configurable Check Interval**: Specify how often to check the process status with a customizable interval.
- **Automatic Shutdown**: Automatically powers off the system when the specified process no longer exists.

## Installation

You can install `afteroff` from [crates.io](https://crates.io/crates/afteroff) using Cargo:

```sh
cargo install afteroff
```

Or clone the repository and build from source:

```sh
git clone https://github.com/yourusername/afteroff.git
cd afteroff
cargo build --release
```

## Usage

```sh
afteroff --pid <PID> [--interval <INTERVAL>]
```

### Parameters

- -p, --pid <PID>: The Process ID to monitor. (required)
- -i, --interval <INTERVAL>: Interval between checks in milliseconds. (default: 5000)

## License

`afteroff` is licensed under the MIT license. See [LICENSE](LICENSE) for more details.


## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
