use anyhow::Result;
use clap::Parser;
use rathole::{run, Cli};
use tokio::{signal, sync::broadcast};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
   /* let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        match listener.accept().await {
            Ok((_socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }*/
    log_init();
    let args = Cli::parse();
    let (shutdown_tx, shutdown_rx) = broadcast::channel::<bool>(1);
    tokio::spawn(async move {
        if let Err(e) = signal::ctrl_c().await{
            panic!("监听ctrl-c信号失败：{:?}",e);
        }
        if let Err(e) = shutdown_tx.send(true){
            panic!("关闭通道推送信令失败！！！");
        }
    });
    run(args, shutdown_rx).await
}

fn log_init(){
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info"))).init();
}
async fn process_socket<TcpStream>(mut socket: tokio::net::TcpStream)
{
    let mut content = String::new();
    socket.read_to_string(&mut content);
    println!("{:?}",content);
    // do work with socket here
}