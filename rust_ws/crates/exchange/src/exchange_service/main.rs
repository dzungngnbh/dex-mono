use crate::lib::env::Env;
use anyhow::Result;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::{AsyncReadExt, FutureExt, StreamExt};
use std::cell::RefCell;
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::rc::Rc;
use tracing::log::info;

// use exchange::pubsub2_capnp::{publisher, subscriber, subscription};

pub mod backend;
pub mod lib;
pub mod ws;

struct SubscriberHandle {
    client: subscriber::Client<::capnp::text::Owned>,
    requests_in_flight: i32,
}

struct SubscriberMap {
    subscribers: HashMap<u64, SubscriberHandle>,
}

impl SubscriberMap {
    fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }
}

struct SubscriptionImpl {
    id: u64,
    subscribers: Rc<RefCell<SubscriberMap>>,
}

impl SubscriptionImpl {
    fn new(id: u64, subscribers: Rc<RefCell<SubscriberMap>>) -> Self {
        Self { id, subscribers }
    }
}

impl Drop for SubscriptionImpl {
    fn drop(&mut self) {
        println!("subscription dropped");
        self.subscribers.borrow_mut().subscribers.remove(&self.id);
    }
}

impl subscription::Server for SubscriptionImpl {}

struct PublisherImpl {
    next_id: u64,
    subscribers: Rc<RefCell<SubscriberMap>>,
}

impl PublisherImpl {
    pub fn new() -> (Self, Rc<RefCell<SubscriberMap>>) {
        let subscribers = Rc::new(RefCell::new(SubscriberMap::new()));
        (
            Self {
                next_id: 0,
                subscribers: subscribers.clone(),
            },
            subscribers,
        )
    }
}

impl publisher::Server<::capnp::text::Owned> for PublisherImpl {
    fn subscribe(
        &mut self,
        params: publisher::SubscribeParams<::capnp::text::Owned>,
        mut results: publisher::SubscribeResults<::capnp::text::Owned>,
    ) -> Promise<(), ::capnp::Error> {
        println!("subscribe");
        self.subscribers.borrow_mut().subscribers.insert(
            self.next_id,
            SubscriberHandle {
                client: pry!(pry!(params.get()).get_subscriber()),
                requests_in_flight: 0,
            },
        );

        results
            .get()
            .set_subscription(capnp_rpc::new_client(SubscriptionImpl::new(
                self.next_id,
                self.subscribers.clone(),
            )));

        self.next_id += 1;
        Promise::ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let env = Env::get_env()?;

    let addr = "127.0.0.1:4000"
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    info!("Starting server on {}", addr);

    tokio::task::LocalSet::new()
        .run_until(async move {
            let listener = tokio::net::TcpListener::bind(&addr).await?;
            let (publisher_impl, subscribers) = PublisherImpl::new();
            let publisher: publisher::Client<_> = capnp_rpc::new_client(publisher_impl);

            let handle_incoming = async move {
                loop {
                    let (stream, _) = listener.accept().await?;
                    stream.set_nodelay(true)?;
                    let (reader, writer) =
                        tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                    let network = twoparty::VatNetwork::new(
                        reader,
                        writer,
                        rpc_twoparty_capnp::Side::Server,
                        Default::default(),
                    );
                    let rpc_system =
                        RpcSystem::new(Box::new(network), Some(publisher.clone().client));

                    tokio::task::spawn_local(rpc_system);
                }
            };

            // Trigger sending approximately once per second.
            let (tx, mut rx) = futures::channel::mpsc::unbounded::<()>();
            std::thread::spawn(move || {
                while let Ok(()) = tx.unbounded_send(()) {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                }
            });

            let send_to_subscribers = async move {
                while let Some(()) = rx.next().await {
                    let subscribers1 = subscribers.clone();
                    let subs = &mut subscribers.borrow_mut().subscribers;
                    for (&idx, subscriber) in subs.iter_mut() {
                        if subscriber.requests_in_flight < 5 {
                            subscriber.requests_in_flight += 1;
                            let mut request = subscriber.client.handle_message_request();
                            request.get().set_message(format!(
                                "system time is: {:?}",
                                ::std::time::SystemTime::now()
                            ))?;
                            let subscribers2 = subscribers1.clone();
                            tokio::task::spawn_local(request.send().promise.map(
                                move |r| match r {
                                    Ok(_) => {
                                        if let Some(ref mut s) =
                                            subscribers2.borrow_mut().subscribers.get_mut(&idx)
                                        {
                                            s.requests_in_flight -= 1;
                                        }
                                    }

                                    Err(_) => {}
                                },
                            ));
                        }
                    }
                }
                Ok::<(), Box<dyn std::error::Error>>(())
            };

            let _: ((), ()) = futures::future::try_join(handle_incoming, send_to_subscribers)
                .await
                .unwrap();
            Ok(())
        })
        .await
}
