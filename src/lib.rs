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
extern crate virt;


#[derive(Debug)]
pub enum Resource {
    Guest {
        name: String,
        memory: u64,
        vcpus: u32,
    },
    Network {
        name: String,
        bridge: String,
        ip_address: String,
        network: String,
    },
    Interface {
        name: String,
        kind: String,
        mac: String,
    },
    StoragePool { name: String, path: String },
}

pub trait Driver {
    fn open(&self);
    fn close(&mut self);
    fn create(&self, resource: &Resource);
    fn clean(&self, resource: &Resource);
}

pub struct Verne<D: Driver, P: Parser> {
    vector: Vec<Resource>,
    driver: D,
    parser: P,
}

impl<D: Driver, P: Parser> Verne<D, P> {
    pub fn new(driver: D, parser: P) -> Verne<D, P> {
        Verne {
            vector: Vec::new(),
            driver: driver,
            parser: parser,
        }
    }

    fn parse(&mut self) {
        self.vector = self.parser.parse();
    }

    pub fn create(&mut self) {
        self.parse();
        self.driver.open();
        for r in &self.vector {
            self.driver.create(r);
        }
        self.driver.close();
    }

    pub fn clean(&mut self) {
        self.parse();
        self.driver.open();
        for r in &self.vector {
            self.driver.clean(r);
        }
        self.driver.close();
    }
}

pub trait Parser {
    fn parse(&self) -> Vec<Resource>;
}

pub trait Command {
    fn execute<D: Driver, P: Parser>(verne: &mut Verne<D, P>);
}

pub mod libvirt;
pub mod yaml;
pub mod command;
