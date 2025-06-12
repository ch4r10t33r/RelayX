use clap::Parser;
use relayx_rpc::RpcServer;
use std::net::{IpAddr, SocketAddr};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct RpcCommand {
    /// The port to run the RPC server on
    #[arg(short, long, default_value_t = 8332)]
    port: u16,

    /// The host to bind the RPC server to
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,
}

impl RpcCommand {
    pub async fn execute(&self) -> anyhow::Result<()> {
        let ip: IpAddr = self.host.parse()?;
        let addr = SocketAddr::new(ip, self.port);

        println!("Starting RPC server on {}", addr);

        let server = RpcServer::new();
        server.start(addr).await?;

        Ok(())
    }
}
