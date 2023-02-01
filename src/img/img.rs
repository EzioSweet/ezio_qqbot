use proc_qq::{
    event, module, GroupMessageEvent, MessageChainAppendTrait, MessageContentTrait,
    MessageSendToSourceTrait, Module, TextEleParseTrait,
};
use proc_qq::re_exports::anyhow;

use crate::util::reply::reply_chain;
use proc_qq::re_exports::anyhow::Context;
use proc_qq::re_exports::regex::Regex;
use proc_qq::re_exports::ricq::msg::MessageChain;

#[event(regexp = "^(\\s+)?随机涩图(\\s+)?$")]
async fn random_image_in_group(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let text = reqwest::get("https://img.xjh.me/random_img.php")
        .await?
        .text()
        .await?;
    let regex = Regex::new("src=\"//(.+\\.jpg)\"")?;
    let mt = regex.captures_iter(&text).next().with_context(|| "Fail")?;
    let real_url = format!("https://{}", mt.get(1).unwrap().as_str());
    let img_src = reqwest::get(real_url).await?.bytes().await?.to_vec();
    let img = event.upload_image_to_source(img_src).await?;
    event
        .send_message_to_source(reply_chain(event).await)
        .await?;
    event
        .send_message_to_source(MessageChain::default().append(img))
        .await?;
    anyhow::Ok(true)
}

#[event(regexp = "^搜涩图 ([\\S\\s]+)?$")]
async fn search_image_in_group(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let content = event.message_content();
    let pid = content.split(' ').collect::<Vec<&str>>()[1].to_string();
    let url = format!("https://pixiv.re/{pid}.jpg");
    let ok_or_not = reqwest::get(url).await;
    match ok_or_not {
        Ok(res) => {
            let ok_or_not_2 = res.bytes().await;
            match ok_or_not_2 {
                Ok(res) => {
                    let img_src = res.to_vec();
                    let ok_or_not_3 = event.upload_image_to_source(img_src).await;
                    match ok_or_not_3 {
                        Ok(img) => {
                            event
                                .send_message_to_source(reply_chain(event).await)
                                .await?;
                            event
                                .send_message_to_source(MessageChain::default().append(img))
                                .await?;
                        }
                        Err(_e) => {
                            event
                                .send_message_to_source(
                                    reply_chain(event)
                                        .await
                                        .append("\n获取错误，请重试".parse_text()),
                                )
                                .await?;
                        }
                    }
                }
                Err(_e) => {
                    event
                        .send_message_to_source(
                            reply_chain(event)
                                .await
                                .append("\n获取错误，请重试".parse_text()),
                        )
                        .await?;
                }
            }
        }
        Err(_e) => {
            event
                .send_message_to_source(
                    reply_chain(event)
                        .await
                        .append("\n获取错误，请重试".parse_text()),
                )
                .await?;
        }
    }

    anyhow::Ok(true)
}

pub fn module() -> Module {
    // id, name, [plugins ...]
    module!("img", "img", random_image_in_group, search_image_in_group)
}
