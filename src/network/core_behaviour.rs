//! Contains core network behaviour that combines all other behaviours

use super::connections_behaviour::ConnectionsBehaviour;
use crate::error::Result;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingConfig, PingEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour,
};
use std::time::Duration;

/// The main behaviour combining all other behaviours.
#[derive(NetworkBehaviour)]
pub struct CoreNetworkBehaviour {
    mdns: Mdns,
    ping: Ping,
    connections: ConnectionsBehaviour,
}

impl NetworkBehaviourEventProcess<()> for CoreNetworkBehaviour {
    fn inject_event(&mut self, _event: ()) {}
}

impl NetworkBehaviourEventProcess<MdnsEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _) in list {
                    log::debug!("Discovered peer: {:?}", peer_id);
                    self.connections.insert_peer(peer_id);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _) in list {
                    // Don't do anything here because ping will drop expired connection
                    log::debug!("Expired peer: {:?}", peer_id);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: PingEvent) {
        match event.result {
            Ok(libp2p::ping::PingSuccess::Ping { rtt }) => {
                log::info!("Ping returned from {} with rtt {:?}", event.peer, rtt)
            }
            Ok(libp2p::ping::PingSuccess::Pong) => log::info!("Sent Pong to {}", event.peer),
            Err(e) => log::error!("{}", e),
        };
    }
}

impl CoreNetworkBehaviour {
    /// Create a new network behavior with `ping` interval
    pub fn new(interval: Duration) -> Result<Self> {
        let mdns = Mdns::new()?;
        let ping = Ping::new(
            PingConfig::new()
                .with_interval(interval)
                .with_keep_alive(true),
        );
        Ok(Self {
            mdns,
            ping,
            connections: ConnectionsBehaviour::new(),
        })
    }
}
