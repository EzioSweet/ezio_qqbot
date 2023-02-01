use proc_qq::re_exports::ricq::client::event::GroupMessageEvent;
use proc_qq::{
    event, module, MessageChainAppendTrait, MessageSendToSourceTrait, Module, TextEleParseTrait,
};
use proc_qq::re_exports::anyhow;

use crate::util::reply::reply_chain;

#[event(regexp = "^(\\s+)?你很好(\\s+)?$")]
async fn group_hello(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    event
        .send_message_to_source(reply_chain(event).await.append("你也很好".parse_text()))
        .await?;
    anyhow::Ok(true)
}
/// 返回一个模块 (向过程宏改进中)
pub fn module() -> Module {
    // id, name, [plugins ...]
    module!("daily", "daily", group_hello)
}
