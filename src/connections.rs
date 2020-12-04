use super::error::Result;
use libp2p::{
    core::connection::{ConnectedPoint, ConnectionId},
    mdns::{Mdns, MdnsEvent},
    swarm::protocols_handler::DummyProtocolsHandler,
    swarm::DialPeerCondition,
    swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters, ProtocolsHandler},
    Multiaddr, PeerId,
};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::{collections::HashSet, time::Duration};

pub struct Connections {
    new_peers: Arc<Mutex<Vec<PeerId>>>,
    remove_peers: Arc<Mutex<Vec<PeerId>>>,
    peers: HashSet<PeerId>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            peers: HashSet::new(),
            new_peers: Arc::new(Mutex::new(vec![])),
            remove_peers: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn insert_peer(&mut self, peer: PeerId) {
        if self.peers.contains(&peer) {
            return;
        }
        self.peers.insert(peer.clone());
        if let Ok(mut new_peers_ref) = self.new_peers.lock() {
            new_peers_ref.push(peer);
        } else {
            log::error!("Poisoned mutex in Connections");
        }
    }

    pub fn remove_peer(&mut self, peer: PeerId) {
        if !self.peers.contains(&peer) {
            return;
        }
        self.peers.remove(&peer);
        if let Ok(mut remove_peers_ref) = self.remove_peers.lock() {
            remove_peers_ref.push(peer);
        } else {
            log::error!("Poisoned mutex in Connections");
        }
    }
}

impl NetworkBehaviour for Connections {
    type ProtocolsHandler = DummyProtocolsHandler;
    type OutEvent = ();

    fn new_handler(&mut self) -> Self::ProtocolsHandler {
        DummyProtocolsHandler::default()
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn inject_connected(&mut self, _: &PeerId) {}

    fn inject_connection_established(&mut self, _: &PeerId, _: &ConnectionId, _: &ConnectedPoint) {}

    fn inject_disconnected(&mut self, _: &PeerId) {}

    fn inject_connection_closed(&mut self, _: &PeerId, _: &ConnectionId, _: &ConnectedPoint) {}

    fn inject_event(
        &mut self,
        _: PeerId,
        _: ConnectionId,
        _: <Self::ProtocolsHandler as ProtocolsHandler>::OutEvent,
    ) {
    }

    fn poll(
        &mut self,
        _: &mut Context<'_>,
        _: &mut impl PollParameters,
    ) -> Poll<
        NetworkBehaviourAction<
            <Self::ProtocolsHandler as ProtocolsHandler>::InEvent,
            Self::OutEvent,
        >,
    > {
        if let Ok(mut new_peers_ref) = self.new_peers.lock() {
            if let Some(peer_id) = new_peers_ref.pop() {
                log::debug!("Connecting peer {:?}", peer_id);
                return Poll::Ready(NetworkBehaviourAction::DialPeer {
                    peer_id,
                    condition: DialPeerCondition::Disconnected,
                });
            }
        } else {
            log::error!("Poisoned mutex in Connections");
        }
        Poll::Pending
    }
}
