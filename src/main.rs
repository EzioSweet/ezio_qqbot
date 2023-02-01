use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use proc_qq::Authentication::{QRCode};
use proc_qq::{ClientBuilder, EventResult, result, ShowQR};
use proc_qq::DeviceSource::JsonFile;
use proc_qq::re_exports::anyhow;
use proc_qq::re_exports::ricq::version::{MACOS};

mod daily;
mod util;
mod img;

#[tokio::main]
async fn main() {
    init_tracing_subscriber();
    ClientBuilder::new()
        .priority_session("session.token")
        .authentication(QRCode)
        .device(JsonFile(String::from("device.json")))
        .version(&MACOS)
        .modules(vec![daily::daily::module(),img::img::module()])
        .result_handlers(vec![on_result{}.into()])
        .show_rq(Some(ShowQR::OpenBySystem))
        .build()
        .await
        .unwrap()
        .start()
        .await
        .unwrap()
        .unwrap();
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
    Ok(false)
}