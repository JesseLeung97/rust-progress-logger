use crate::config;
use crate::sheets_reader;
use serde;
use reqwest;
use chrono::Datelike;

const SLACK_URL: &str = "https://slack.com/api/chat.postMessage";

#[derive(serde::Serialize)]
struct SlackMessage {
    channel: String,
    text: String
}

pub async fn send_message_ok(config: &config::Config, curr_work: &sheets_reader::CurrentWork) {
    let bot_token = config.slack_bot_token.clone();
    let text = format_message(curr_work);

    let msg = SlackMessage {
        channel: config.slack_channel_id.clone(),
        text
    };


    let client = reqwest::Client::new();
    if let Err(err) = client.post(SLACK_URL)
        .header("AUTHORIZATION", format!("Bearer {bot_token}"))
        .json(&msg)
        .send()
        .await
    {
        send_message_err(config, err.to_string()).await;
    };
}

pub async fn send_message_err(config: &config::Config, err: String) {
    let bot_token = config.slack_bot_token.clone();
    let err_text = format!("Encountered the following error while generating the daily report:\n{err}");

    let msg = SlackMessage {
        channel: config.slack_error_channel_id.clone(),
        text: err_text
    };

    let client = reqwest::Client::new();
    let res = client.post(SLACK_URL)
        .header("AUTHORIZATION", format!("Bearer {bot_token}"))
        .json(&msg)
        .send()
        .await;

    match res {
        Err(err) => {
            let err = format!("{:?}", err);
            panic!("{}", err);
        },
        Ok(res) => println!("{:?}", res)
    };

}

fn format_message(curr_work: &sheets_reader::CurrentWork) -> String {
    let date = date_today_to_string();
    let yest = if let Some(task_list) = &curr_work.yesterday {
        vec_to_string(&task_list)
    } else {
        "".to_string()
    };
    let today = if let Some(task_list) = &curr_work.today {
        vec_to_string(&task_list)
    } else {
        "".to_string()
    };

    format!("{date}\n■前日の作業内容{yest}■当日の作業予定{today}\n■作業内容や進捗についての問題点\n■その他連絡事項")
}

fn date_today_to_string() -> String {
    let curr_time = chrono::offset::Local::now();
    let year = curr_time.year().to_string();
    let month = curr_time.month().to_string();
    let day = curr_time.day().to_string();

    format!("{year}{month}{day}")
}

fn vec_to_string(task_list: &Vec<String>) -> String {
    task_list.iter()
        .map(|task| format!("- {task}\n"))
        .reduce(|curr, next| curr + &next)
        .unwrap()
}
