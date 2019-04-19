//Copyright Venire Labs Inc 2019 All rights reserved

//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and


pub use super::Metrics::StreamDriverStatistics;

use crate::interlock::daten::async::deferred_column;


pub trait SinkStreamExecutor: SendEvent{

    fn schema(&self)->&[ImmuType]

}