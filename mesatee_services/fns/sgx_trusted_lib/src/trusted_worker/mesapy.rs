// Copyright 2019 MesaTEE Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Insert std prelude in the top for the sgx feature
#[cfg(feature = "mesalock_sgx")]
use std::prelude::v1::*;

use crate::trait_defs::{WorkerHelper, WorkerInput};
use itertools::Itertools;
use mesatee_core::{Error, ErrorKind, Result};
use std::format;

const MAXPYBUFLEN: usize = 1024;
const MESAPY_ERROR_BUFFER_TOO_SHORT: i64 = -1i64;
const MESAPY_EXEC_ERROR: i64 = -2i64;

extern "C" {
    fn mesapy_exec(input: *const u8, output: *mut u8, buflen: u64) -> i64;
}

pub fn mesapy_from_buffer(_helper: &mut WorkerHelper, input: WorkerInput) -> Result<String> {
    let payload = match input.payload {
        Some(value) => value,
        None => return Err(Error::from(ErrorKind::MissingValue)),
    };

    let mut py_script_vec =
        base64::decode(&payload).or_else(|_| Err(Error::from(ErrorKind::InvalidInputError)))?;
    py_script_vec.push(0u8);
    let mut py_result = [0u8; MAXPYBUFLEN];

    let result = unsafe {
        mesapy_exec(
            py_script_vec.as_ptr(),
            &mut py_result as *mut _ as *mut u8,
            MAXPYBUFLEN as u64,
        )
    };

    match result {
        MESAPY_ERROR_BUFFER_TOO_SHORT => Ok("MESAPY_ERROR_BUFFER_TOO_SHORT".to_string()),
        MESAPY_EXEC_ERROR => Ok("MESAPY_EXEC_ERROR".to_string()),
        len => {
            let r: Vec<u8> = py_result.iter().take(len as usize).copied().collect();
            let payload = format!("marshal.loads(b\"\\x{:02X}\")", r.iter().format("\\x"));
            Ok(payload)
        }
    }
}
