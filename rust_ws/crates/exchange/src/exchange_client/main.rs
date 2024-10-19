use anyhow::Result;
use capnp::capability::Promise;
use capnp_rpc::{pry, rpc_twoparty_capnp, twoparty, RpcSystem};
use futures::AsyncReadExt;
use std::net::ToSocketAddrs;

use exchange::pubsub2_capnp::{publisher, subscriber};

struct SubscriberImpl;

impl subscriber::Server<::capnp::text::Owned> for SubscriberImpl {
    fn handle_message(
        &mut self,
        params: subscriber::HandleMessageParams<::capnp::text::Owned>,
        _results: subscriber::HandleMessageResults<::capnp::text::Owned>,
    ) -> Promise<(), ::capnp::Error> {
        println!(
            "message from publisher: {}",
            pry!(pry!(pry!(params.get()).get_message()).to_str())
        );
        Promise::ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    let addr = "127.0.0.1:4000"
        .to_socket_addrs()?
        .next()
        .expect("could not parse address");

    tokio::task::LocalSet::new()
        .run_until(async move {
            let stream = tokio::net::TcpStream::connect(&addr).await?;
            stream.set_nodelay(true)?;
            let (reader, writer) =
                tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
            let rpc_network = Box::new(twoparty::VatNetwork::new(
                reader,
                writer,
                rpc_twoparty_capnp::Side::Client,
                Default::default(),
            ));
            let mut rpc_system = RpcSystem::new(rpc_network, None);
            let publisher: publisher::Client<::capnp::text::Owned> =
                rpc_system.bootstrap(rpc_twoparty_capnp::Side::Server);
            let sub = capnp_rpc::new_client(SubscriberImpl);

            let mut request = publisher.subscribe_request();
            request.get().set_subscriber(sub);

            // Need to make sure not to drop the returned subscription object.
            futures::future::try_join(rpc_system, request.send().promise).await?;
            Ok(())
        })
        .await
}
