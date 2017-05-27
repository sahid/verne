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

use std::fs::File;
use std::io::prelude::*;

use yaml_rust::YamlLoader;

use Parser;
use Resource;


#[derive(Debug)]
pub struct YamlParser {
    pub template: String,
}

impl YamlParser {
    fn get_template_content(&self) -> String {
        let mut file = File::open(&self.template).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    }
}

impl Parser for YamlParser {
    fn parse(&self) -> Vec<Resource> {
        let tpl = self.get_template_content();
        let docs = YamlLoader::load_from_str(&tpl).unwrap();
        let doc = &docs[0];
        let mut vector: Vec<Resource> = Vec::new();
        for (key, values) in doc.as_hash().unwrap() {
            match key.as_str().unwrap() {
                "guest" => {
                    for guest in values.as_vec().unwrap() {
                        vector.push(Resource::Guest {
                                        name: guest["name"].as_str().unwrap().to_owned(),
                                        memory: guest["memory"].as_i64().unwrap() as u64,
                                        vcpus: guest["vcpus"].as_i64().unwrap() as u32,
                                    })
                    }
                }
                _ => warn!("Resource '{:?}' not implemented", key),
            }
        }
        vector
    }
}
