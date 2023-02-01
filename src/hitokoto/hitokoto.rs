use std::collections::HashMap;
use proc_qq::re_exports::ricq::client::event::GroupMessageEvent;
use proc_qq::{
    event, module, MessageChainAppendTrait, MessageSendToSourceTrait, Module, TextEleParseTrait,
};
use proc_qq::re_exports::anyhow;
use serde_derive::{Deserialize, Serialize};

use crate::util::reply::reply_chain;

#[derive(Serialize,Deserialize)]
struct Hitokoto{
    hitokoto:String,
    from:String,
}

#[event(regexp = "^(\\s+)?随机一言(\\s+)?$")]
async fn random_hitokoto(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let map = reqwest::get("https://v1.hitokoto.cn/?c=b&c=a&c=c")
        .await?
        .json::<Hitokoto>()
        .await?;

    event
        .send_message_to_source(
            reply_chain(event)
                .await
                .append("\n".parse_text())
                .append(map.hitokoto.parse_text())
                .append("\n".parse_text())
                .append("---".parse_text())
                .append(map.from.parse_text())
        ).await?;
    anyhow::Ok(true)
}
/// 返回一个模块 (向过程宏改进中)
pub fn module() -> Module {
    // id, name, [plugins ...]
    module!("hitokoto", "hitokoto", random_hitokoto)
}
