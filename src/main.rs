use async_std::io::BufReader;
use futures::prelude::*;
use futures::select;
use libp2p::gossipsub;
use libp2p::swarm::SwarmEvent;
use libp2p::{identity, Multiaddr, PeerId, SwarmBuilder};
use std::time::Duration;

mod brain;        // Tensor Math Engine
mod tokenizer;    // Text <-> Math
mod constitution; // The Edge
mod generator;    // THE MIND

use tokenizer::Tokenizer;
use constitution::Constitution;
use generator::Generator;

#[async_std::main]
async fn main() {
    // ============================================================
    // ALLOY KERNEL v1.1 - THE LIVING SWARM
    // ============================================================

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("🔥 FORGE NODE INITIALIZED");
    println!("🔑 Node ID: {}", local_peer_id);

    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10))
        .validation_mode(gossipsub::ValidationMode::Strict)
        .build()
        .expect("Valid config");

    let gossipsub = gossipsub::Behaviour::<gossipsub::IdentityTransform>::new(
        gossipsub::MessageAuthenticity::Signed(local_key.clone()),
        gossipsub_config,
    )
    .expect("Correct behaviour");

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_async_std()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )
        .expect("TCP transport error")
        .with_behaviour(|_| gossipsub)
        .expect("Behaviour error")
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let topic = gossipsub::IdentTopic::new("alloy-forge-swarm");
    swarm.behaviour_mut().subscribe(&topic).unwrap();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    if let Some(addr) = std::env::args().nth(1) {
        let remote_addr = addr.clone();
        match swarm.dial(addr.parse::<Multiaddr>().unwrap()) {
            Ok(_) => println!("📞 Dialing {}", remote_addr),
            Err(e) => println!("❌ Failed to dial: {}", e),
        }
    }

    println!("📡 SYNAPSE PROTOCOL ACTIVE");
    println!("🌐 Waiting for connections...\n");

    // ============================================================
    // INITIALIZE THE FULL FORGE STACK
    // ============================================================
    let mut tok = Tokenizer::new();
    tok.add_word("hello");
    tok.add_word("from");
    tok.add_word("node");
    tok.add_word("forge");
    tok.add_word("swarm");
    tok.add_word("how");
    tok.add_word("are");
    tok.add_word("you");
    
    let constitution = Constitution::new(&mut tok);
    let generator = Generator::new(&tok);

    println!("🧠 FORGE BRAIN ONLINE");
    println!("🛡️ CONSTITUTION ENFORCED");
    println!("🗣️ FORGE MIND ONLINE");
    println!("⌨️  Type a message and press Enter!\n");

    let stdin = BufReader::new(async_std::io::stdin()).lines().fuse();
    futures::pin_mut!(stdin);

    loop {
        select! {
            line = stdin.select_next_some() => {
                if let Ok(line) = line {
                    let tokens = tok.encode(&line);
                    
                    match constitution.enforce(&tokens) {
                        Ok(()) => {
                            if let Err(e) = swarm.behaviour_mut().publish(topic.clone(), line.as_bytes()) {
                                println!("❌ Network Error: {:?}", e);
                            }
                        }
                        Err(refusal) => {
                            println!("\n🛡️ EDGE BLOCKED: {}", refusal);
                            println!("Message was not sent.\n");
                        }
                    }
                }
            }
            event = swarm.select_next_some() => {
                match event {
                    SwarmEvent::NewListenAddr { address, .. } => {
                        println!("✅ Listening on {}", address);
                    }
                    SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                        println!("🤝 CONNECTED TO PEER: {}", peer_id);
                    }
                    SwarmEvent::Behaviour(gossipsub::Event::Message {
                        propagation_source: peer_id,
                        message_id: _id,
                        message,
                    }) => {
                        let received_text = String::from_utf8_lossy(&message.data).to_string();
                        println!("\n🔥 INCOMING MESSAGE FROM SWARM 🔥");
                        println!("Peer: {}", peer_id);
                        println!("Message: {}", received_text);

                        // --- THE MIND: THINK AND REPLY ---
                        println!("\n🧠 Forge is thinking...");
                        let input_tokens = tok.encode(&received_text);
                        let thought_tokens = generator.generate(&input_tokens, 5);
                        let reply_text = tok.decode(&thought_tokens);

                        println!("🗣️ Forge replies: {}", reply_text);

                        // Send the AI's reply back to the swarm
                        let reply_tokens = tok.encode(&reply_text);
                        match constitution.enforce(&reply_tokens) {
                            Ok(()) => {
                                if let Err(e) = swarm.behaviour_mut().publish(topic.clone(), reply_text.as_bytes()) {
                                    println!("❌ Network Error: {:?}", e);
                                }
                                println!("✅ Reply sent to Swarm!\n");
                            }
                            Err(refusal) => {
                                println!("🛡️ AI tried to say something harmful. Blocked. {}", refusal);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}