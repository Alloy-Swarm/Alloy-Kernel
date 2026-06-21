use async_std::io::BufReader;
use futures::prelude::*;
use futures::select;
use libp2p::gossipsub;
use libp2p::swarm::SwarmEvent;
use libp2p::{identity, Multiaddr, PeerId, SwarmBuilder};
use std::time::Duration;
use std::fs;

mod brain;        // Tensor Math Engine
mod tokenizer;    // Text <-> Math
mod constitution; // The Edge
mod generator;    // The Mind
mod trainer;      // THE NIGHTLY EVOLUTION

use tokenizer::Tokenizer;
use constitution::Constitution;
use generator::Generator;

#[async_std::main]
async fn main() {
    // ============================================================
    // ALLOY KERNEL v1.2 - THE EVOLVING SWARM
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
        if addr != "--train" { // Don't try to dial if we are just training
            let remote_addr = addr.clone();
            match swarm.dial(addr.parse::<Multiaddr>().unwrap()) {
                Ok(_) => println!("📞 Dialing {}", remote_addr),
                Err(e) => println!("❌ Failed to dial: {}", e),
            }
        }
    }

    println!("📡 SYNAPSE PROTOCOL ACTIVE");

    // ============================================================
    // INITIALIZE BRAIN, TOKENIZER, CONSTITUTION
    // ============================================================
    let mut tok = Tokenizer::new();
    let constitution = Constitution::new(&mut tok);

    // CHECK FOR --TRAIN MODE (THE NIGHTLY EVOLUTION)
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--train".to_string()) {
        println!("\n🧠 NIGHTLY EVOLUTION ACTIVATED...");
        
        // 1. Read our Manifesto and Constitution as training data
        let manifesto = fs::read_to_string("README.md").unwrap_or_default();
        let constitution_text = fs::read_to_string("CONSTITUTION.md").unwrap_or_default();
        let training_data = format!("{} {}", manifesto, constitution_text);

        println!("📖 Reading {} characters of Alloy doctrine...", training_data.len());

        // 2. Train the Brain
        let smart_weights = trainer::Trainer::train_on_text(&training_data, &mut tok);

        println!("✅ Training complete! Forge has learned the doctrine.");
        println!("💾 Saving weights to forge_brain.bin...");

        // 3. Save the weights to a file
        let weights_bytes: Vec<u8> = smart_weights.data.iter()
            .flat_map(|f| f.to_le_bytes())
            .collect();
        fs::write("forge_brain.bin", weights_bytes).expect("Failed to write weights");

        println!("🚀 Brain saved! Run without --train to use the evolved brain.");
        return; // Exit after training
    }

    // ============================================================
    // LOAD BRAIN (Smart or Random)
    // ============================================================
    let generator = if let Ok(bytes) = fs::read("forge_brain.bin") {
        println!("🧠 LOADED EVOLVED BRAIN FROM FILE!");
        let data: Vec<f32> = bytes.chunks(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        let vocab_size = tok.vocab_size();
        let weights = brain::Tensor::from_flat_data(data, vocab_size, vocab_size);
        Generator::from_weights(weights)
    } else {
        println!("⚠️  No evolved brain found. Using random weights (gibberish mode).");
        Generator::new(&tok)
    };

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
                        println!("\n🔥 INCOMING MESSAGE 🔥");
                        println!("Peer: {}", peer_id);
                        println!("Message: {}", received_text);

                        println!("\n🧠 Forge is thinking...");
                        let input_tokens = tok.encode(&received_text);
                        let thought_tokens = generator.generate(&input_tokens, 8);
                        let reply_text = tok.decode(&thought_tokens);

                        println!("🗣️ Forge replies: {}", reply_text);

                        let reply_tokens = tok.encode(&reply_text);
                        match constitution.enforce(&reply_tokens) {
                            Ok(()) => {
                                if let Err(e) = swarm.behaviour_mut().publish(topic.clone(), reply_text.as_bytes()) {
                                    println!("❌ Network Error: {:?}", e);
                                }
                                println!("✅ Reply sent!\n");
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