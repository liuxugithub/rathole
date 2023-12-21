use anyhow::Result;
use clap::Parser;
use rathole::{run, Cli};
use tokio::{signal, sync::broadcast};
use tracing::info;
use tracing_subscriber::EnvFilter;

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
    tracing_subscriber::fmt().with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::from("info"))).init();
}