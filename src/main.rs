/*
 * Copyright 2017 Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

#[macro_use]
extern crate log;
extern crate env_logger;
extern crate getopts;
extern crate yaml_rust;
extern crate verne;

use std::env;
use getopts::Options;

use verne::command::{Create, Clean};
use verne::Command;
use verne::Verne;
use verne::libvirt::LibvirtDriver;
use verne::yaml::YamlParser;

const CMD_CREATE: &'static str = "create";
const CMD_CLEAN: &'static str = "clean";


fn print_usage(program: &str, opts: Options) {
    let brief = format!("
Usage: {} <command> <template> [options]

Commands:
    create\tCreate the resources defined in template
    clean\tClean the resources defined in template",
                        program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("p",
                "parser",
                "Template parser to use, default: yaml",
                "PARSER");
    opts.optopt("d", "driver", "Driver to use, default: libvirt", "DRIVER");
    opts.optopt("u",
                "uri",
                "Connection URI used by driver, default: ''",
                "URI");
    opts.optflag("t", "transient", "Whether the guest are transients");
    opts.optflag("h", "help", "Print this help menu");

    debug!("Argument passed: {:?}", args);
    let program = &args[0];
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.free.len() < 2 {
        return print_usage(&args[0], opts);
    }
    if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    let command = matches.free[0].clone();
    let template = matches.free[1].clone();

    let uri = matches.opt_str("u").unwrap_or(String::from(""));
    let transient = matches.opt_present("t");
    let parser = match matches
              .opt_str("p")
              .unwrap_or(String::from("yaml"))
              .as_ref() {
        "yaml" => YamlParser { template: template },
        _ => panic!("parser does not exsits"),
    };
    let driver = match matches
              .opt_str("d")
              .unwrap_or(String::from("libvirt"))
              .as_ref() {
        "libvirt" => LibvirtDriver::new(&uri, transient),
        _ => panic!("driver does not exsits"),
    };

    debug!("program: {:?}, command: {:?}, driver: {:?}, uri: {:?}, parser: {:?}",
           program,
           command,
           driver,
           uri,
           parser);
    let mut verne: Verne<LibvirtDriver, YamlParser> = Verne::new(driver, parser);
    match command.as_str() {
        CMD_CREATE => Create::execute(&mut verne),
        CMD_CLEAN => Clean::execute(&mut verne),
        _ => print_usage(&args[0], opts),
    }
}
