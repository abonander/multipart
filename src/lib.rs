// Copyright 2016 `multipart` Crate Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//! Client- and server-side abstractions for HTTP `multipart/form-data` requests.
//!
//! Features: 
//! 
//! * `client` (default): Enable the client-side abstractions for multipart requests. If the
//! `hyper` feature is also set, enables integration with the Hyper HTTP client API.
//!
//! * `server` (default): Enable the server-side abstractions for multipart requests. If the
//! `hyper` feature is also set, enables integration with the Hyper HTTP server API.
//!
//! * `hyper` (default): Enable integration with the [Hyper](https://github.com/hyperium/hyper) HTTP library 
//! for client and/or server depending on which other feature flags are set.
//!
//! * `iron`: Enable integration with the [Iron](http://ironframework.io) web application
//! framework. See the [`server::iron`](server/iron/index.html) module for more information.
//!
//! * `tiny_http`: Enable integration with the [`tiny_http`](https://github.com/frewsxcv/tiny-http)
//! crate. See the [`server::tiny_http`](server/tiny_http/index.html) module for more information.
//!
//! * `nickel_`: Enable integration with the [Nickel](http://nickel.rs) web application framework.
//! See the [`server::nickel`](server/nickel/index.html) module for more information. Enables the `hyper`
//! feature.
#![warn(missing_docs)]

// Needs to be at crate root
#[cfg(all(test, feature = "client", feature = "server"))]
#[macro_use]
extern crate log;

#[cfg(feature = "client")]
#[doc(hidden)]
pub extern crate multipart_client;

#[cfg(feature = "server")]
#[doc(hidden)]
pub extern crate multipart_server;

pub use multipart_client as client;
pub use multipart_server as server;

#[cfg(all(test, feature = "client", feature = "server"))]
mod local_test;


