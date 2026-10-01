use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

use teloxide::{
    prelude::*, sugar::request, types::{
        InlineKeyboardButton, InlineKeyboardButtonKind::Url, InlineKeyboardMarkup, InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText, MenuButton, WebAppInfo,
    },
};

const MINI_APP_URL: &str = "https://boon-code.github.io/telegram-mini-app-test/index.html";
const COUNTER_QUERY_PREFIX: &str = "counter:";

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    let counter = Arc::new(AtomicI64::new(0));

    println!("Bot started...");

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![counter])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn message_handler(bot: Bot, msg: Message, counter: Arc<AtomicI64>) -> ResponseResult<()> {
    let Some(text) = msg.text() else {
        return respond(());
    };

    let command = text.split_whitespace().next().unwrap_or_default();
    let command = command.split('@').next().unwrap_or_default();

    match command {
        "/start" => {
            if msg.chat.is_private() {

                let button = InlineKeyboardButton::web_app("Open Mini App", WebAppInfo { url: MINI_APP_URL.parse().unwrap() });

                bot.send_message(msg.chat.id, "Open the Mini App:")
                    .reply_markup(InlineKeyboardMarkup::new([[button]]))
                    .await?;
            } else {
                let me = bot.get_me().await?;
                if me.has_main_web_app {
                    let mut mini_app_url = me.tme_url();
                    mini_app_url.set_query(Some("startapp"));

                    let button = InlineKeyboardButton::url("Open Mini App", mini_app_url);
                    bot.send_message(msg.chat.id, "Open the Mini App:")
                        .reply_markup(InlineKeyboardMarkup::new([[button]]))
                        .await?;
                } else {
                    bot.send_message(
                        msg.chat.id,
                        "Group launch is not configured yet. Set up a Main Mini App for this bot in @BotFather, then try /start again.",
                    )
                    .await?;
                }
            }
        }
        "/show" => {
            let value = counter.load(Ordering::Relaxed);
            bot.send_message(msg.chat.id, format!("Current counter: {value}"))
                .await?;
        }
        _ => {}
    }

    respond(())
}

async fn inline_query_handler(
    bot: Bot,
    query: InlineQuery,
    counter: Arc<AtomicI64>,
) -> ResponseResult<()> {
    let results = query
        .query
        .strip_prefix(COUNTER_QUERY_PREFIX)
        .and_then(|value| value.parse::<i64>().ok())
        .map(|value| {
            counter.store(value, Ordering::Relaxed);
        })
        .unwrap_or_default();

    respond(())
}
