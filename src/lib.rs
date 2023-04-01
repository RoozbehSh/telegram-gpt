use wa_rust_telegram_bot::{Api, Config, UpdateHandler, UpdateKind};
use openai_flows::{chat_completion, ChatOptions, ChatModel};
use std::env;

#[no_mangle]
pub fn run() {
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    let telegram_token = std::env::var("telegram_token").unwrap();
    let api = Api::new(&telegram_token);

    let target_channel_id = -1772546492; // replace with the ID of your target channel

    let handler = UpdateHandler::new().handle(move |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text.unwrap_or("");
            let chat_id = msg.chat.id();

            // Check if the user is in the target channel
            let user_in_channel = api.get_chat_member(target_channel_id, msg.from.id).map(|member| {
                member.status == wa_rust_telegram_bot::types::ChatMemberStatus::Member
                    || member.status == wa_rust_telegram_bot::types::ChatMemberStatus::Administrator
                    || member.status == wa_rust_telegram_bot::types::ChatMemberStatus::Creator
            });

            match user_in_channel {
                Ok(true) => {
                    let prompt =
                        "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
                    let co = ChatOptions {
                        model: ChatModel::GPT35Turbo,
                        restart: text.eq_ignore_ascii_case("restart"),
                        restarted_sentence: Some(prompt),
                    };

                    let c = chat_completion(&openai_key_name, &chat_id.to_string(), &text, &co);
                    if let Some(c) = c {
                        if c.restarted {
                            _ = api.send_message(chat_id, "I am starting a new conversation since it has been over 10 minutes from your last reply. You can also tell me to restart by typing \"restart\" into the chat.\n\n".to_string() + &c.choice);
                        } else {
                            _ = api.send_message(chat_id, c.choice);
                        }
                    }
                }
                _ => {
                    _ = api.send_message(chat_id, "Please join the target channel to get your questions answered.".to_string());
                }
            }
        }
        Ok(())
    });

    Config::new().update_handler(handler).run();
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
