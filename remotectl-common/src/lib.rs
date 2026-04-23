//! Common data structures for the remote controller.

use std::net::SocketAddr;

use clap::Parser;
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

/// Command for the rocket launcher.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Command {
    /// Stop any previous action.
    Stop,

    /// Yaw left.
    Left,

    /// Yaw right.
    Right,

    /// Pitch up.
    Up,

    /// Pitch down.
    Down,

    /// Fire missiles.
    Fire,
}

/// Command line arguments for client and server.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Parser)]
pub struct Args {
    proxy: SocketAddr,
}

impl Args {
    /// Return the proxy's socket address.
    #[must_use]
    pub const fn proxy(&self) -> SocketAddr {
        self.proxy
    }

    /// Return the URL to the proxy.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] if the URL failed to parse.
    pub fn url(&self) -> Result<Url, ParseError> {
        format!("http://{}:{}/", self.proxy.ip(), self.proxy.port()).parse()
    }
}
