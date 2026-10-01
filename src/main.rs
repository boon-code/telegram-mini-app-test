use teloxide::{
    prelude::*,
    types::{ButtonRequest, KeyboardButton, KeyboardMarkup, WebAppInfo},
};

const MINI_APP_URL: &str = "https://boon-code.github.io/telegram-mini-app-test/index.html";

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();

    println!("Bot started...");

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // /start -> show the Mini App button.
        if msg.text() == Some("/start") {
            let button = KeyboardButton {
                text: "🚀 Open Mini App".to_string(),
                request: Some(ButtonRequest::WebApp(WebAppInfo {
                    url: MINI_APP_URL.parse().unwrap(),
                })),
            };

            let keyboard = KeyboardMarkup::new([[button]])
                .resize_keyboard();

            bot.send_message(
                msg.chat.id,
                "Open the Mini App:",
            )
            .reply_markup(keyboard)
            .await?;

            return Ok(());
        }

        // Data sent by Telegram.WebApp.sendData(...)
        if let Some(data) = msg.web_app_data() {
            bot.send_message(
                msg.chat.id,
                format!("Mini App sent: {}", data.data),
            )
            .await?;

            return Ok(());
        }

        Ok(())
    })
    .await;
}
