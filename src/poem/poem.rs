use std::collections::HashMap;
use std::time::Duration;
use proc_qq::{event, GroupMessageEvent, MessageChainAppendTrait, MessageContentTrait, MessageSendToSourceTrait, Module, module, TextEleParseTrait};
use proc_qq::re_exports::anyhow;
use reqwest::header::HeaderMap;
use serde_derive::{Deserialize, Serialize};
use crate::util::reply::reply_chain;

#[event(regexp = "^藏头诗 ([\\S\\s]+)?$")]
async fn arousic(event: &GroupMessageEvent) -> anyhow::Result<bool> {
    let client = reqwest::Client::new();
    let content = event.message_content();
    let args = content.split(' ').collect::<Vec<&str>>();
    let poem_count = args[1];
    let poem_type = args[2];
    let poem_word=args[3];
    let count = match poem_count {
        "五言"=>"5",
        _=>"7"
    };
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent","Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:109.0) Gecko/20100101 Firefox/110.0".parse().unwrap());
    let mut data = HashMap::new();
    data.insert("yan", count);
    data.insert("poem", poem_word);
    data.insert("sentiment", poem_type);
    let mut x = client.post("http://166.111.5.188:12315/jiugepoem/task/send_arousic")
        .headers(headers.clone())
        .json(&data)
        .send()
        .await?
        .json::<HashMap<String,String>>()
        .await?;
    x.remove("code");
    tokio::time::sleep(Duration::from_secs(1)).await;
    let poem = client.post("http://166.111.5.188:12315/jiugepoem/task/get_arousic")
        .headers(headers)
        .json(&x)
        .send()
        .await?
        .json::<Poem>()
        .await?;
    event
        .send_message_to_source(
            reply_chain(event)
                .await
                .append("\n".parse_text())
                .append(poem.output[0].clone().parse_text())
                .append("\n".parse_text())
                .append(poem.output[1].clone().parse_text())
                .append("\n".parse_text())
                .append(poem.output[2].clone().parse_text())
                .append("\n".parse_text())
                .append(poem.output[3].clone().parse_text())
        ).await?;
    anyhow::Ok(true)
}
#[derive(Debug,Deserialize,Serialize)]
struct Poem{
    output:Vec<String>,
    title:String,
    status:String
}
pub fn module() -> Module {
    // id, name, [plugins ...]
    module!("poem", "poem", arousic)
}
