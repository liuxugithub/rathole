use std::time::Duration;
use sha2::digest::typenum::Mod;
use time::macros::format_description;
use time::UtcOffset;
use tokio::io::AsyncReadExt;
use tokio::signal;
use tokio::sync::{broadcast};
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::time::OffsetTime;

#[tokio::test]
async fn test_server(){
    log_init();
    let (mut shutdown_tx,mut  shutdown_rx) = broadcast::channel::<bool>(1);
    tokio::spawn(async move{
        for i in(0..100){
            info!("i:{}",i);
            tokio::time::sleep(Duration::from_secs(1)).await;
            if let Ok(bool) = shutdown_tx.send(true){
                info!("推送:{}",i);
            }
        }
    });
    loop{
        tokio::select! {
            ret =  shutdown_rx.recv()=>{
                if let Ok(bool) = ret{
                     info!("收到关闭指令:{:?}",ret);
                }
            }
        }
    }
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