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

use Verne;
use Command;
use Driver;
use Parser;

pub struct Create;
pub struct Clean;


impl Command for Create {
    fn execute<D: Driver, P: Parser>(verne: &mut Verne<D, P>) {
        debug!("Executing command create...");

        verne.create()
    }
}

impl Command for Clean {
    fn execute<D: Driver, P: Parser>(verne: &mut Verne<D, P>) {
        debug!("Executing command clean...");

        verne.clean()
    }
}
