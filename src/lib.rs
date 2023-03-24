use tg_flows::{listen_to_update, Telegram, UpdateKind};
use openai_flows::{chat_completion, ChatOptions, ChatModel};
use tg_flows::{listen_to_update, Telegram, UpdateKind};
use std::env;
use tg_botapi::{Api, GetChatMember, ChatId};

/*
#[no_mangle]
pub fn run() {
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());
    let api = Api::new(&telegram_token);
    
    let channel_username = "ruzuntu";
    let chat_id = ChatId::Username(channel_username.to_owned());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;
            
            let user_id = msg.from.id;
            
            let request = GetChatMember::new(chat_id, user_id);
            match api.send(&request).await {
                Ok(chat_member) => {
                    // The user is a member of the channel.
                    // You can answer their question here.
                    _ = tele.send_message(chat_id, "You are a member of the channel!".to_string() + &text);
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
                    
                },
                Err(err) => {
                    // The user is not a member of the channel.
                    // You can ask them to join the channel here.
                    // You can also check the specific error returned by the API to handle cases where the user is banned, etc.
                    _ = tele.send_message(chat_id, "Please join our channel @ruzuntu to ask questions!".to_string() + &text);
                }
            }


        }
    });
}
*/
#[no_mangle]
pub fn run() {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let api = Api::new(&telegram_token);

    // Replace "CHANNEL_USERNAME" with the username of your channel.
    let channel_username = "ruzuntu";
    let chat_id = ChatId::Username(channel_username.to_owned());

    listen_to_update(telegram_token, |update| {
        if let UpdateKind::Message(msg) = update.kind {
            let text = msg.text().unwrap_or("");
            let chat_id = msg.chat.id;

            let request = GetChatMember::new(chat_id, msg.from.id);
            match api.send(&request).await {
                Ok(chat_member) => {
                    // The user is a member of the channel.
                    // You can answer their question here.
                    let prompt = "You are a helpful persian/English assistant answering questions on Telegram.\n\n If someone greets you without asking a question, you can simply respond \"Hello, I am your assistant on Telegram, built by the Ruzuntu team. I am ready for your question now!\nسلام! من ربات تیم روزونتو هستم! چگونه می توانم به شما کمک کنم؟\" \n\n".to_owned() + &text + "\n```";
                    let co = ChatOptions {
                        model: ChatModel::GPT35Turbo,
                        restart: text.eq_ignore_ascii_case("restart"),
                        restarted_sentence: Some(&prompt),
                    };
                    let c = chat_completion(&openai_key_name, &chat_id.to_string(), &text, &co);
                    if let Some(c) = c {
                        if c.restarted {
                            api.send(SendMessage::new(chat_id, "I am starting a new conversation since it has been over 10 minutes from your last reply. You can also tell me to restart by typing \"restart\" into the chat.\n\n".to_string() + &c.choice)).await;
                        } else {
                            api.send(SendMessage::new(chat_id, c.choice)).await;
                        }
                    }
                },
                Err(err) => {
                    // The user is not a member of the channel.
                    // You can ask them to join the channel here.
                    // You can also check the specific error returned by the API to handle cases where the user is banned, etc.
                    api.send(SendMessage::new(chat_id, "Please join our channel @ruzuntu to ask questions!".to_string() + &text)).await;
                }
            }
        }
        Ok(())
    });
}
