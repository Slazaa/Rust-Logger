use std::io::Write;
use std::fs::OpenOptions;

use chrono::{Local, Timelike, Datelike};

#[derive(Debug)]
pub enum LogType {
	Debug,
	Info,
	Warn,
	Error,
	Fatal
}

pub fn log(log_type: LogType, message: &str) {
	let mut file = OpenOptions::new()
		.write(true)
		.append(true)
		.create(true)
		.open("log.txt")
		.unwrap();

	let now = Local::now();

	writeln!(file, "({}-{}-{}) ({}:{}:{}) [{:?}] - {}",
		now.year(), now.month(), now.day(),
		now.hour(), now.minute(), now.second(),
		log_type,
		message
	).unwrap();
}