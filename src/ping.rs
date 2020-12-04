mod handler;

use super::error::Result;
use handler::PingHandler;
use libp2p::{
    core::connection::{ConnectedPoint, ConnectionId},
    mdns::{Mdns, MdnsEvent},
    swarm::protocols_handler::DummyProtocolsHandler,
    swarm::{NetworkBehaviour, NetworkBehaviourAction, PollParameters, ProtocolsHandler},
    Multiaddr, PeerId,
};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::{collections::HashSet, time::Duration};

pub struct Ping {
    incoming_peers: Arc<Mutex<Vec<PeerId>>>,
    peers: HashSet<PeerId>,
}

impl Ping {
    pub fn new() -> Self {
        Self {
            peers: HashSet::new(),
            incoming_peers: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl NetworkBehaviour for Ping {
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
        Poll::Pending
    }
}
