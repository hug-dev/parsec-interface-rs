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
//! # PARSEC Rust Interface
//!
//! The PARSEC Rust Interface provides methods to communicate easily with the PARSEC service using
//! the [wire protocol](https://github.com/docker/parsec/blob/master/docs/wire_protocol.md).
//!
//! ## For the PARSEC service
//!
//! This library is used by the PARSEC service to:
//! * read from a stream a `Request` sent to the service with the `read_from_stream` method
//! * use the `body_to_operation` method of the `Convert` trait on a converter to parse the request
//! body into a `NativeOperation`
//!
//! execute the operation to yield a `NativeResult` and:
//! * use the `result_to_body` method to serialize the `NativeResult`
//! * create a `Response` containing the result as its body and write it back to the stream  with
//! the `write_to_stream` method.
//!
//! ### Example
//!
//!```no_run
//!use std::os::unix::net::UnixStream;
//!use parsec_interface::operations::{Convert, NativeResult};
//!use parsec_interface::requests::{Request, Response};
//!use parsec_interface::operations_protobuf::ProtobufConverter;
//!use parsec_interface::operations::ResultCreateKey;
//!
//!let mut stream = UnixStream::connect("socket_path").unwrap();
//!let converter = ProtobufConverter {};
//!let request = Request::read_from_stream(&mut stream).unwrap();
//!let operation = converter.body_to_operation(request.body, request.header.opcode).unwrap();
//!
//!// Deal with the operation to get a `NativeResult`
//!let result = NativeResult::CreateKey(ResultCreateKey {});
//!let result_body = converter.result_to_body(result).unwrap();
//!let response = Response {
//!    header: request.header.into(),
//!    body: result_body,
//!};
//!response.write_to_stream(&mut stream).unwrap();
//!```
//!
//! ## For the PARSEC Rust clients
//!
//! This library is used by the PARSEC Rust clients to:
//! * use the `operation_to_body` method to serialize the `NativeOperation` to be sent as body of a
//! `Request`
//! * write it to the stream with the `write_to_stream` method.
//!
//! and after the operation has been executed by the PARSEC service:
//! * read from a stream the `Response` from the service with the `read_from_stream` method
//! * use the `body_to_result` method to parse the result body into a `NativeResult`
//!
//! See the [PARSEC Test client](https://github.com/docker/parsec-client-test) as an example of a
//! Rust client.
//!
//! ### Example
//!
//!```no_run
//!use std::os::unix::net::UnixStream;
//!use parsec_interface::operations::{Convert, NativeOperation};
//!use parsec_interface::requests::{Request, Response, ProviderID, BodyType, AuthType, Opcode};
//!use parsec_interface::requests::request::{RequestHeader, RequestAuth};
//!use parsec_interface::operations_protobuf::ProtobufConverter;
//!use parsec_interface::operations::OpPing;
//!
//!let mut stream = UnixStream::connect("socket_path").unwrap();
//!let converter = ProtobufConverter {};
//!let operation = NativeOperation::Ping(OpPing {});
//!let request = Request {
//!    header: RequestHeader {
//!        version_maj: 0,
//!        version_min: 0,
//!        provider: ProviderID::CoreProvider,
//!        session: 0,
//!        content_type: BodyType::Protobuf,
//!        accept_type: BodyType::Protobuf,
//!        auth_type: AuthType::Simple,
//!        opcode: Opcode::Ping,
//!    },
//!    body: converter.operation_to_body(operation).unwrap(),
//!    auth: RequestAuth::from_bytes(Vec::new()),
//!};
//!request.write_to_stream(&mut stream).unwrap();
//!
//!// Wait for the service to execute the operation
//!let response = Response::read_from_stream(&mut stream).unwrap();
//!let result = converter.body_to_result(response.body, response.header.opcode).unwrap();
//!```

pub mod operations;
pub mod operations_protobuf;
pub mod requests;
