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

use virt::connect::Connect;
use virt::domain::Domain;
use virt::error::Error;

use Driver;
use Resource;


#[derive(Debug)]
pub struct LibvirtDriver {
    conn: Connect,
    transient: bool,
    kind: String,
}

impl Driver for LibvirtDriver {
    fn open(&self) {}
    fn close(&mut self) {
        assert_eq!(Ok(0), self.conn.close());
    }
    fn create(&self, resource: &Resource) {
        info!("Creating resource: {:?}...", resource);
        match resource {
            &Resource::Guest {
                 ref name,
                 ref vcpus,
                 ref memory,
             } => self.create_guest(name, vcpus, memory),
            _ => warn!("Resource '{:?}' not implemented", resource),
        }
    }

    fn clean(&self, resource: &Resource) {
        info!("Cleaning resource: {:?}...", resource);
        match resource {
            &Resource::Guest {
                 ref name,
                 ref vcpus,
                 ref memory,
             } => self.clean_guest(name),
            _ => warn!("Resource '{:?}' not implemented", resource),
        }

    }
}

impl LibvirtDriver {
    pub fn new(uri: &str, transient: bool) -> LibvirtDriver {
        match Connect::open(uri) {
            Ok(conn) => {
                let kind = match conn.get_type() {
                    Ok(k) => k,
                    Err(e) => {
                        panic!("Unable to retrieve conn type, error code {}, message: {}",
                               e.code,
                               e.message)
                    }
                };
                LibvirtDriver {
                    conn: conn,
                    transient: transient,
                    kind: kind.to_lowercase(),
                }
            }
            Err(e) => {
                panic!("Unable to connect, error code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }

    fn create_guest(&self, name: &String, vcpus: &u32, memory: &u64) {
        let xml = match self.kind.as_str() {
            "qemu" => self.default_guest_xml(name, vcpus, memory),
            "test" => self.default_guest_xml(name, vcpus, memory),
            "lxc" => self.lxc_guest_xml(name, vcpus, memory),
            _ => panic!("unsuported domain type"),
        };
        debug!("Creating guest, transient: {}, xml: {}",
               self.transient,
               xml);
        let result: Result<Domain, Error>;
        if self.transient {
            result = Domain::create_xml(&self.conn, &xml, 0);
        } else {
            result = Domain::define_xml(&self.conn, &xml);
        }

        debug!("Created, now starting {}...", name);
        match result {
            Ok(dom) => {
                dom.create().expect("Domain not started");
            }
            Err(e) => {
                warn!("Was not able to create/define domain, code: {}, message: {}",
                      e.code,
                      e.message)
            }
        };
    }

    fn clean_guest(&self, name: &String) {
        debug!("Cleaining guest {}...", name);

        match Domain::lookup_by_name(&self.conn, name) {
            Ok(mut dom) => {
                dom.destroy();
                if !self.transient {
                    dom.undefine();
                }
                dom.free();
            }
            Err(e) => {
                warn!("Domain not found, code: {}, message: {}", e.code, e.message);
            }
        };
    }

    fn default_guest_xml(&self, name: &String, vcpus: &u32, memory: &u64) -> String {
        let xml = format!("<domain type='{}'>
                             <name>{}</name>
                             <memory unit='KiB'>{}</memory>
                             <vcpus>{}</vcpus>
                             <features>
                               <acpi/>
                               <apic/>
                             </features>
                             <os>
                               <type>hvm</type>
                             </os>
                           </domain>",
                          self.kind,
                          name,
                          memory,
                          vcpus);
        xml
    }

    fn lxc_guest_xml(&self, name: &String, vcpus: &u32, memory: &u64) -> String {
        let xml = format!("<domain type='{}'>
                             <name>{}</name>
                             <memory unit='KiB'>{}</memory>
                             <vcpus>{}</vcpus>
                             <os>
                               <type>exe</type>
                               <init>/bin/sh</init>
                             </os>
                             <devices>
                               <console type='pty'/>
                             </devices>
                           </domain>",
                          self.kind,
                          name,
                          memory,
                          vcpus);
        xml
    }
}
