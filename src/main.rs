// Copyright (c) 2020 Steve Jones
// SPDX-License-Identifier: MIT

use clap::{App, Arg};
use modaws::Service;
use serde_json::Result as JsResult;
use std::error::Error;
use std::fs;
use std::sync::atomic::Ordering;

fn main() -> Result<(), Box<dyn Error>>{
    let matches = App::new("modaws")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("compact")
            .long("compact")
            .help("Compact JSON output"))
        .arg(Arg::with_name("yaml")
            .long("yaml")
            .help("YAML output"))
        .arg(Arg::with_name("strip-documentation")
            .long("strip-documentation")
            .help("Remove documentation from model"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();
    let file_name = matches.value_of("INPUT").unwrap();
    let file_content = fs::read_to_string(file_name)?;
    let file_service_result : JsResult<Service> = serde_json::from_str(&file_content);
    if matches.is_present("strip-documentation") {
        modaws::SER_SKIP_DOCUMENTATION.store(true, Ordering::Relaxed);
    }
    let text = if matches.is_present("yaml") {
        serde_yaml::to_string(&file_service_result?)?
    } else if matches.is_present("compact") {
        serde_json::to_string(&file_service_result?)?
    } else {
        serde_json::to_string_pretty(&file_service_result?)?
    };
    println!("{}", text);
    Ok(())
}
