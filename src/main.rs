mod connections;
mod error;
mod network;

use error::Result;
use futures::stream::StreamExt;
use libp2p::identity::{secp256k1::Keypair, PublicKey};
use libp2p::{PeerId, Swarm};
use network::CoreNetworkBehaviour;
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<()> {
    env_logger::init();
    start_network().await?;
    Ok(())
}

async fn start_network() -> Result<()> {
    log::info!("Starting p2p network");
    let (keypair, peer_id) = generate_secret();
    let libp2p_keypair = libp2p::identity::Keypair::Secp256k1(keypair);
    let transport = libp2p::build_development_transport(libp2p_keypair)?;
    let behaviour = CoreNetworkBehaviour::new(Duration::from_secs(10))?;

    let mut swarm = Swarm::new(transport, behaviour, peer_id);
    Swarm::listen_on(&mut swarm, "/ip4/0.0.0.0/tcp/0".parse()?)?;
    let swarm = swarm.for_each(|_ev| futures::future::ready(()));
    log::info!("Network started");
    swarm.await;
    Ok(())
}

pub fn generate_secret() -> (Keypair, PeerId) {
    let keypair = Keypair::generate();
    let public_key = PublicKey::Secp256k1(keypair.public().clone());
    let peer_id = PeerId::from_public_key(public_key);
    (keypair, peer_id)
}
