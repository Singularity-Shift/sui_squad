use anyhow::Result;
use sled::Db;
use squad_connect::client::squad_connect::SquadConnect;
use sui_squad_core::{
    ai::ResponsesClient, 
    commands::bot_commands::Command,
    conversation::ConversationCache
};
use teloxide::{prelude::*, types::Message, utils::command::BotCommands, Bot};

use crate::bot_manage::handlers::{handle_fund, handle_login};

use super::handlers::{handle_prompt};

pub async fn answer(
    bot: Bot,
    msg: Message,
    cmd: Command,
    responses_client: ResponsesClient,
    squad_connect_client: SquadConnect,
    conversation_cache: ConversationCache,
    db: Db,
) -> Result<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Login => handle_login(bot, msg, db).await?,
        Command::Fund => handle_fund(bot, msg, squad_connect_client).await?,
        Command::Prompt(prompt_text) => handle_prompt(
            bot, 
            msg, 
            prompt_text, 
            responses_client, 
            squad_connect_client,
            conversation_cache,
            db,
        ).await?,
        Command::P(prompt_text) => handle_prompt(
            bot, 
            msg, 
            prompt_text, 
            responses_client, 
            squad_connect_client,
            conversation_cache,
            db
        ).await?,
        Command::PromptExamples => bot.send_message(msg.chat.id, "Here are some example prompts you can use:\n\n💰 Wallet & Balance:\n- /prompt \"What's my wallet address?\" or /p \"What's my wallet address?\"\n- /prompt \"Show my balance\" or /p \"Show my balance\"\n- /prompt \"Check my SUI balance\" or /p \"Check my SUI balance\"\n- /prompt \"How much do I have?\" or /p \"How much do I have?\"\n\n💸 Transactions:\n- /prompt \"Send 10 SUI to @username\" or /p \"Send 10 SUI to @username\"\n- /prompt \"Withdraw 5 SUI\" or /p \"Withdraw 5 SUI\"\n- /prompt \"Send 100 SUI to everyone\" or /p \"Send 100 SUI to everyone\"\n\n❓ General:\n- /prompt \"What can you help me with?\" or /p \"What can you help me with?\"\n- /prompt \"Explain how this bot works\" or /p \"Explain how this bot works\"\n\n💡 Tip: Use /p as a shortcut for /prompt!").await?,
    };
    Ok(())
}
