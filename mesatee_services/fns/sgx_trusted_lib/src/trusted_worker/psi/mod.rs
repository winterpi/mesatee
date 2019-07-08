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
use mesatee_core::{Error, ErrorKind, Result};

mod basic;
mod compute;
use compute::SetIntersection;

pub fn psi(helper: &mut WorkerHelper, input: WorkerInput) -> Result<String> {
    if input.input_files.len() != 2 {
        return Err(Error::from(ErrorKind::InvalidInputError));
    }

    let file1 = &input.input_files[0];
    let file2 = &input.input_files[1];

    let plaintext1 = helper.read_file(&file1)?;
    let plaintext2 = helper.read_file(&file2)?;

    let mut si = SetIntersection::new();
    if !si.psi_add_hash_data(plaintext1, 0) {
        return Err(Error::from(ErrorKind::InvalidInputError));
    }
    if !si.psi_add_hash_data(plaintext2, 1) {
        return Err(Error::from(ErrorKind::InvalidInputError));
    }
    si.compute();
    let _result_file1 = helper.save_file_for_file_owner(&si.data[0].result, file1)?;
    let _result_file2 = helper.save_file_for_file_owner(&si.data[1].result, file2)?;

    Ok("Finished".to_string())
}
