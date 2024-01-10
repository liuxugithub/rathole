use anyhow::Result;
use clap::Parser;
use time::macros::format_description;
use time::UtcOffset;
use rathole::{run, Cli};
use tokio::{signal, sync::broadcast};
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{fmt::time::OffsetTime, EnvFilter};
#[tokio::main]
async fn main() -> Result<()> {
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
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info")))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]"),
        )).init();
}
async fn process_socket<TcpStream>(mut socket: tokio::net::TcpStream)
{
    let mut content = String::new();
    socket.read_to_string(&mut content);
    println!("{:?}",content);
    // do work with socket here
}