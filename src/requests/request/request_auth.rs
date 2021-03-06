// Copyright (c) 2019, Arm Limited, All Rights Reserved
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//          http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use crate::requests::Result;
use std::io::{Read, Write};

/// Wrapper around the body of a request.
///
/// Hides the contents and keeps them immutable.
#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RequestAuth {
    bytes: Vec<u8>,
}

impl RequestAuth {
    /// Construct new, empty request authentication field.
    /// Available for testing only.
    #[cfg(feature = "testing")]
    pub(super) fn new() -> RequestAuth {
        RequestAuth { bytes: Vec::new() }
    }

    /// Read a request authentication field from the stream, given the length
    /// of the byte stream contained.
    pub(super) fn read_from_stream(mut stream: &mut impl Read, len: usize) -> Result<RequestAuth> {
        let bytes = get_from_stream!(stream; len);
        Ok(RequestAuth { bytes })
    }

    /// Write request authentication field to stream.
    pub(super) fn write_to_stream(&self, stream: &mut impl Write) -> Result<()> {
        stream.write_all(&self.bytes)?;
        Ok(())
    }

    /// Create a `RequestAuth` from a vector of bytes.
    pub fn from_bytes(bytes: Vec<u8>) -> RequestAuth {
        RequestAuth { bytes }
    }

    /// Get the auth as a slice of bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the size of the auth field.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    // Check if auth field is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
