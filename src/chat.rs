use futures::stream::StreamExt;
use gossipsub::IdentTopic;
use libp2p::{
    Swarm, gossipsub, mdns, noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tokio::{io, select, sync::mpsc};

use tracing_subscriber::EnvFilter;

use crate::event::{AppEvent, Event};

#[derive(NetworkBehaviour)]
struct CustomBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[derive(Default, Debug)]
struct UserInfo {
    name: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
struct Message {
    sender_name: String,
    message: String,
}

pub struct ChatHandler {
    /// Channel used to send messages to thread for publishing
    sender: mpsc::UnboundedSender<Message>,
    /// Current user info
    user_info: UserInfo,
}

impl ChatHandler {
    /// Constructs a new [`ChatHandler`] and spawns a new thread to handle chat messages
    pub fn new(event_tx: mpsc::UnboundedSender<Event>) -> Self {
        let user_info = UserInfo {
            name: "None".to_string(),
        };

        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = ChatThread::new(event_tx, receiver);
        tokio::spawn(async { actor.expect("Error spawning chat thread!").run().await });
        Self { sender, user_info }
    }

    pub fn send(&self, text: String) {
        let message = Message {
            sender_name: self.user_info.name.clone(),
            message: text,
        };
        self.sender
            .send(message)
            .expect("Chat thread dropped too early!");
    }

    pub fn update_name(&mut self, name: String) {
        self.user_info.name = name;
    }
}

struct ChatThread {
    swarm: Swarm<CustomBehaviour>,
    topic: IdentTopic,
    /// Channel to send message events to the main app
    sender: mpsc::UnboundedSender<Event>,
    /// Channel to receive messages the handler wants to be published
    receiver: mpsc::UnboundedReceiver<Message>,
}

impl ChatThread {
    fn new(
        sender: mpsc::UnboundedSender<Event>,
        receiver: mpsc::UnboundedReceiver<Message>,
    ) -> color_eyre::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .try_init();

        let mut swarm = libp2p::SwarmBuilder::with_new_identity()
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )?
            .with_quic()
            .with_behaviour(|key| {
                // content-address messages by taking hash of the message as an id
                let message_id_fn = |message: &gossipsub::Message| {
                    let mut s = DefaultHasher::new();
                    message.data.hash(&mut s);
                    gossipsub::MessageId::from(s.finish().to_string())
                };

                let gossipsub_config = gossipsub::ConfigBuilder::default()
                    .heartbeat_interval(Duration::from_secs(10))
                    .validation_mode(gossipsub::ValidationMode::Strict) // enforce message signing
                    .message_id_fn(message_id_fn)
                    .build()
                    .map_err(io::Error::other)?;

                // build a gossipsub network behavior using the config defined above
                let gossipsub = gossipsub::Behaviour::new(
                    gossipsub::MessageAuthenticity::Signed(key.clone()),
                    gossipsub_config,
                )?;

                let mdns = mdns::tokio::Behaviour::new(
                    mdns::Config::default(),
                    key.public().to_peer_id(),
                )?;
                Ok(CustomBehaviour { gossipsub, mdns })
            })?
            .build();

        // create a gossipsub topic
        let topic = gossipsub::IdentTopic::new("TermFishing");
        // subscribe the swarm to our topic
        swarm.behaviour_mut().gossipsub.subscribe(&topic)?;

        // listen on all interfaces and OS-assigned port
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(Self {
            swarm,
            topic,
            sender,
            receiver,
        })
    }

    async fn run(&mut self) -> color_eyre::Result<()> {
        loop {
            select! {
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(CustomBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                        for (peer_id, _multiaddr) in list { // new peer connected
                            // println!("mDNS discovered a new peer: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(CustomBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                        for (peer_id, _multiaddr) in list { // peer disconnected
                            // println!("mDNS discover peer has expired: {peer_id}");
                            self.swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        }
                    },
                    SwarmEvent::Behaviour(CustomBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                        propagation_source: _peer_id,
                        message_id: _id,
                        message,
                    })) => { // message received
                        let msg: Message = rmp_serde::from_slice(&message.data)?;
                        let text = format!("{}: {}", msg.sender_name, msg.message);
                        let _ = self.sender.send(Event::App(AppEvent::MessageReceived(text)));
                    },
                    // SwarmEvent::NewListenAddr { address, .. } => { // listen addr swapped
                    //     // println!("Local node is listening on {address}");
                    // },
                    _ => {}
                },
                message = self.receiver.recv() => match message {
                    Some(m) => self.publish(m)?,
                    None => {},
                }
            }
        }
    }

    /// Publishes a message into the chat
    fn publish(&mut self, message: Message) -> color_eyre::Result<()> {
        let buf = rmp_serde::to_vec(&message);

        match &buf {
            Err(_e) => {
                // TODO: add proper error handling?
                // let text = format!("Serializer error: {e:?}");
                // let _ = self
                //     .sender
                //     .send(Event::App(AppEvent::MessageReceived(text)));
            }
            _ => {}
        }

        if let Err(_e) = self
            .swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.topic.clone(), buf.unwrap())
        {
            // TODO: add proper error handling?
            // let text = format!("Publish error: {e:?}");
            // let _ = self
            //     .sender
            //     .send(Event::App(AppEvent::MessageReceived(text)));
        }
        Ok(())
    }
}
