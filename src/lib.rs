use std::env;
use tg_flows::{listen_to_update, Telegram, UpdateKind};
use openai_flows::{chat_completion, ChatModel, ChatOptions};

#[no_mangle]
pub fn run() {
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    let channel_username = "ruzuntu";

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;
            let user_id = msg.from.id;
            
            // Check if user is a member of the channel
            let is_member = tele.is_member_of_channel(user_id, channel_username);
            
            if !is_member {
                // Prompt user to join the channel
                _ = tele.send_message(chat_id, "Please join our channel to access this service.".to_string() + &channel_username);
                return;
            }
            
            // User is a member of the channel, proceed with chat
            let prompt = "You are a helpful assistant answering questions on Telegram.\n\nIf someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Second State team. I am ready for your question now!\"";
            let co = ChatOptions {
                model: ChatModel::GPT35Turbo,
                restart: text.eq_ignore_ascii_case("restart"),
                restarted_sentence: Some(prompt),
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
