//! P2P pinging network. First command line argument specifies ping interval.
//!
//! E.g. `RUST_LOG=info cargo run -- 5` will ping all other peers every 5 seconds.
//! The default interval is 10 secs.

mod error;
mod network;

use error::Result;
use network::start_network;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();
    let interval_secs = if let Some(interval_str) = std::env::args().nth(1) {
        if let Ok(interval) = interval_str.parse() {
            interval
        } else {
            log::warn!("Failed to parse interval, applying 10 secs");
            10
        }
    } else {
        10
    };
    start_network(interval_secs).await?;
    Ok(())
}
