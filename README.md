# Rust - Logger
A Rust cross-platform logger

## Overview
It creates a log.txt file in the current directory and write each log with this pattern:
`(data) (time) [type] - message`

## Example
```rs
use rust_logger::log::{LogType, log};

fn main() {
	log(LogType::Info, "Info!");
	log(LogType::Warn, "Warn!");
	log(LogType::Error, "Error!");
	log(LogType::Fatal, "Fatal!");
}
```

## Libraries Used
* [chrono](https://github.com/chronotope/chrono): Date and Time for Rust