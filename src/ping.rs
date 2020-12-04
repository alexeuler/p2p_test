use libp2p::{
    mdns::{Mdns, MdnsEvent},
    swarm::NetworkBehaviourEventProcess,
    NetworkBehaviour, PeerId,
};
use std::collections::HashSet;

pub struct Ping {
    peers: HashSet<PeerId>,
}

impl Ping {
    pub fn new() -> Self {
        Self {
            peers: HashSet::new(),
        }
    }
}
