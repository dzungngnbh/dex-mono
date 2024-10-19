fn main() -> Result<(), Box<dyn std::error::Error>> {
    capnpc::CompilerCommand::new().file("pubsub.capnp").run()?;
    capnpc::CompilerCommand::new().file("pubsub2.capnp").run()?;
    Ok(())
}
