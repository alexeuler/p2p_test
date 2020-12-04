use super::connections::Connections;
use super::error::Result;
use libp2p::{
    mdns::{Mdns, MdnsEvent},
    ping::{Ping, PingConfig, PingEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId,
};
use std::{collections::HashSet, time::Duration};

#[derive(NetworkBehaviour)]
pub struct CoreNetworkBehaviour {
    pub mdns: Mdns,
    pub ping: Ping,
    pub connections: Connections,
    #[behaviour(ignore)]
    peers: HashSet<PeerId>,
}

impl NetworkBehaviourEventProcess<()> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: ()) {}
}

impl NetworkBehaviourEventProcess<MdnsEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _) in list {
                    log::debug!("Discovered peer: {:?}", peer_id);
                    self.connections.insert_peer(peer_id);
                }
                log::debug!("Updated peer set: {:?}", self.peers)
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, _) in list {
                    log::debug!("Expired peer: {:?}", peer_id);
                    self.peers.remove(&peer_id);
                }
                log::debug!("Updated peer set: {:?}", self.peers)
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
    pub fn new(duration: Duration) -> Result<Self> {
        let mdns = Mdns::new()?;
        let ping = Ping::new(
            PingConfig::new()
                .with_interval(duration)
                .with_keep_alive(true),
        );
        Ok(Self {
            mdns,
            ping,
            connections: Connections::new(),
            peers: HashSet::new(),
        })
    }
}
