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
//
// SPDX-License-Identifier: Apache-2.0

//! OpenStack API SDK
//!
//! This SDK provides synchronous and asynchronous interfaces to
//! communicate with OpenStack based clouds.
//!
//! The simplest example demonstrating how to list compute flavors:
//! ```rust
//! use openstack_sdk::api::{paged, Pagination, QueryAsync};
//! use openstack_sdk::{AsyncOpenStack, config::ConfigFile, OpenStackError};
//! use openstack_sdk::types::ServiceType;
//! use openstack_sdk::api::compute::v2::flavor::list;
//!
//! async fn list_flavors() -> Result<(), OpenStackError> {
//!     // Get the builder for the listing Flavors Endpoint
//!     let mut ep_builder = list::Request::builder();
//!     // Set the `min_disk` query param
//!     ep_builder.min_disk("15");
//!     let ep = ep_builder.build().unwrap();
//!
//!     let cfg = ConfigFile::new().unwrap();
//!     // Get connection config from clouds.yaml/secure.yaml
//!     let profile = cfg.get_cloud_config("devstack".to_string()).unwrap().unwrap();
//!     // Establish connection
//!     let mut session = AsyncOpenStack::new(&profile).await?;
//!
//!     // Invoke service discovery when desired.
//!     session.discover_service_endpoint(&ServiceType::Compute).await?;
//!
//!     // Execute the call with pagination limiting maximum amount of entries to 1000
//!     let data: Vec<serde_json::Value> = paged(ep, Pagination::Limit(1000))
//!         .query_async(&session)
//!         .await.unwrap();
//!
//!     println!("Data = {:?}", data);
//!     Ok(())
//! }
//! ```
//!
//! SDK currently implements following features:
//!
//! - Every single API operation is a separate module (generated from OpenAPI
//! spec)
//!
//! - paged combinator allows fetching all/limited entries from endpoints
//! supporting pagination
//!
//! - find combinator allows combining GET and LIST calls to find entity by
//! identificator
//!
//! - client is responsible/capable of passing structure results should be
//! deserialized into. While it is possible to use types coming with SDK
//! directly it allows also ignoring unnecessary fields as well as requesting
//! fields not yet supported by the SDK itself.
//!
//! - sync/async interface
pub mod api;
mod auth;
mod catalog;
pub mod config;
mod openstack;
mod state;
mod utils;
mod websso;

pub mod types;

pub use crate::auth::AuthError;
pub use crate::openstack::{AsyncOpenStack, OpenStack, OpenStackError, RestError};

#[cfg(test)]
#[allow(dead_code)]
mod test;
