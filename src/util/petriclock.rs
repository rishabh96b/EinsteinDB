//Copyright 2019 Venire Labs Inc

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::util::time::{monotonic_raw_now, Instant};
use std::cmp::{Ord, Ordering, Reverse};
use std::collections::BinaryHeap;
use std::sync::{mpsc, Arc};
use std::thread::Builder;
use std::time::Duration;
use tokio_executor::park::ParkThread;
use tokio_timer::{self, clock::Clock, clock::Now, timer::Handle, Delay};

pub struct PetriClock<T> {

    pending: BinaryHeap<Reverse<TimeOutTask<T>>>

}