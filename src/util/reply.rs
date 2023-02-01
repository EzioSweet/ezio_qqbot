use proc_qq::re_exports::ricq::msg::MessageChain;
use proc_qq::re_exports::ricq_core::msg::elem::At;
use proc_qq::{GroupMessageEvent, MessageChainAppendTrait, TextEleParseTrait};

pub async fn reply_chain(group: &GroupMessageEvent) -> MessageChain {
    let mut at = At::new(group.inner.from_uin);
    at.display = format!("@{}", group.inner.group_card);
    MessageChain::default().append(at).append("\n".parse_text())
}
