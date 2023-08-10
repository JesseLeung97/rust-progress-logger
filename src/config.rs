use std::env;
use dotenv;

pub struct Config {
    pub slack_bot_token: String,
    pub slack_channel_id: String,
    pub slack_error_channel_id: String,
    pub priv_key: String,
    pub sheets_id: String,
    pub deposit_range_input: String
}

impl Config {
    pub fn init() -> Config {
        dotenv::dotenv().ok();

        let slack_bot_token = env::var("SLACK_BOT_TOKEN").expect("The BOT_TOKEN env variable is not set.");
        let slack_channel_id = env::var("SLACK_CHANNEL_ID").expect("The SLACK_CHANNEL_ID env variable is not set.");
        let slack_error_channel_id = env::var("SLACK_ERROR_CHANNEL_ID").expect("The SLACK_ERROR_CHANNEL_ID env variable is not set.");
        let priv_key = String::from("priv_key.json");
        let sheets_id = env::var("SHEETS_ID").expect("The SHEETS_ID env variable is not set.");
        let deposit_range_input = env::var("DEPOSIT_RANGE_INPUT").expect("The DEPOSIT_RANGE_INPUT env variable is not set.");

        Config {
            slack_bot_token,
            slack_channel_id, 
            slack_error_channel_id,
            priv_key,
            sheets_id,
            deposit_range_input
        }
    }
}
