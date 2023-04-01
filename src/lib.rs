use std::{env, error::Error};
use telegram_bot::{Api, CanSendMessage, UpdateKind};
use telegram_bot_raw::get_chat_member::Request;
use tg_futures::stream::StreamExt;
use tg_futures::FutureExt;
use openai_flows::{chat_completion, ChatModel, ChatOptions};
use futures::future::TryFutureExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_key_name = match env::var("openai_key_name") {
        Ok(name) => name,
        Err(_) => "chatmichael".to_string(),
    };

    let telegram_token = std::env::var("telegram_token").expect("TELEGRAM_TOKEN is not set.");
    let api = Api::new(telegram_token.clone());

    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        if let Ok(update) = update {
            match update.kind {
                UpdateKind::Message(message) => {
                    let text = message.text().unwrap_or("");
                    let chat_id = message.chat.id();

                    let target_channel = -1772546492; // Replace with the ID of your target channel.
                    let request = Request::new(target_channel, message.from.id);
                    let chat_member = api.send(request).await?;

                    if chat_member.is_member() || chat_member.is_administrator() || chat_member.is_creator() {
                        let prompt = "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
                        let co = ChatOptions {
                            model: ChatModel::GPT35Turbo,
                            restart: text.eq_ignore_ascii_case("restart"),
                            restarted_sentence: Some(prompt),
                        };

                        let c = chat_completion(&openai_key_name, &chat_id.to_string(), &text, &co)
                            .await
                            .unwrap();
                        if c.restarted {
                            api.send(
                                chat_id.text(format!(
                                    "I am starting a new conversation since it has been over 10 minutes from your last reply. You can also tell me to restart by typing \"restart\" into the chat.\n\n{}",
                                    c.choice
                                ))
                                .parse_mode(tg_botapi::types::ParseMode::Html),
                            )
                            .await?;
                        } else {
                            api.send(chat_id.text(c.choice).parse_mode(tg_botapi::types::ParseMode::Html)).await?;
                        }
                    } else {
                        api.send(chat_id.text("Please join the target channel to get your questions answered.")).await?;
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}


/*
use tg_flows::{listen_to_update, Telegram, UpdateKind};
use openai_flows::{chat_completion, ChatOptions, ChatModel};
use std::env;

#[no_mangle]
pub fn run() {
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;

            let prompt = "You are a helpful assistant answering questions on Telegram.\n\n If someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\" \n\n".to_owned() + &text + "\n```";
            let co = ChatOptions {
                model: ChatModel::GPT35Turbo,
                restart: text.eq_ignore_ascii_case("restart"),
                restarted_sentence: Some(&prompt)
            };

            let c = chat_completion(&openai_key_name, &chat_id.to_string(), &text, &co);
            if let Some(c) = c {
                if c.restarted {
                    _ = tele.send_message(chat_id, "I am starting a new conversation since it has been over 10 minutes from your last reply. You can also tell me to restart by typing \"restart\" into the chat.\n\n".to_string() + &c.choice);
                } else {
                    _ = tele.send_message(chat_id, c.choice);
                }
            }
        }
    });
}
*/
