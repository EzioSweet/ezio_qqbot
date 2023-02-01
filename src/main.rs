use std::sync::Arc;
use std::time::Duration;
use proc_qq::Authentication::QRCode;
use proc_qq::DeviceSource::JsonFile;
use proc_qq::{result, ClientBuilder, EventResult, ShowQR, run_client, MessageChainParseTrait};
use proc_qq::re_exports::anyhow;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use proc_qq::re_exports::ricq::version::MACOS;


mod daily;
mod img;
mod util;
mod hitokoto;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    let client = ClientBuilder::new()
        .priority_session("session.token")
        .authentication(QRCode)
        .device(JsonFile(String::from("device.json")))
        .version(&MACOS)
        .modules(vec![daily::daily::module(), img::img::module(),hitokoto::hitokoto::module()])
        .result_handlers(vec![on_result {}.into()])
        .show_rq(Some(ShowQR::OpenBySystem))
        .build()
        .await
        .unwrap();
    run_client(client).await.unwrap();
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .without_time(),
        )
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_target("ricq", Level::DEBUG)
                .with_target("proc_qq", Level::DEBUG)
                // 这里改成自己的crate名称
                .with_target("ezio_qqbot", Level::DEBUG),
        )
        .init();
}

#[result]
pub async fn on_result(result: &EventResult) -> anyhow::Result<bool> {
    match result {
        EventResult::Process(info) => {
            tracing::info!("{} : {} : 处理了一条消息", info.module_id, info.handle_name);
        }
        EventResult::Exception(info, err) => {
            tracing::info!(
                "{} : {} : 遇到了错误 : {}",
                info.module_id,
                info.handle_name,
                err
            );
        }
    }
    anyhow::Ok(false)
}
