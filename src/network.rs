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
    #[behaviour(ignore)]
    peers: HashSet<PeerId>,
}

impl NetworkBehaviourEventProcess<MdnsEvent> for CoreNetworkBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, _) in list {
                    log::debug!("Discovered peer: {:?}", peer_id);
                    self.peers.insert(peer_id);
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

impl CoreNetworkBehaviour {
    pub fn new(duration: Duration) -> Result<Self> {
        let mdns = Mdns::new()?;
        Ok(Self {
            mdns,
            peers: HashSet::new(),
        })
    }
}
