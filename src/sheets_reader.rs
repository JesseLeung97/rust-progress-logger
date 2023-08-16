use chrono::Datelike;
use google_sheets4;
use serde_json;
use crate::config;

pub struct CurrentWork {
    pub yesterday: Option<Vec<String>>,
    pub today: Option<Vec<String>>
}

pub async fn read(
    config: &config::Config,
    hub: &google_sheets4::Sheets<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>
) -> Result<(hyper::Response<hyper::Body>, google_sheets4::api::ValueRange), google_sheets4::Error> {
    hub.spreadsheets()
        .values_get(&config.sheets_id, &config.sheet_range)
        .doit()
        .await
}

// TODO grab previous month's sheet if yesterday is less than 0
pub fn get_current_work(data: Vec<Vec<serde_json::Value>>) -> Result<CurrentWork, Box<dyn std::error::Error>> {
    let today = get_day();
    let yesterday = if today > 0 { today - 1 } else { 0 };

    let mut curr_work = CurrentWork {
        yesterday: None,
        today: None
    };

    for row in data.iter() {
        let day_num = serde_json::from_value::<String>(row[1].clone())?;
        let day_num = if day_num.trim().is_empty() { 0 } else { day_num.parse::<u32>()? };
        if day_num == today {
            let task_list = get_daily_task_list(row)?;
            if let Some(tasks) = csv_to_vec(task_list) {
                curr_work.today = Some(tasks);
            }

            return Ok(curr_work);
        }

        if day_num == yesterday {
            let task_list = get_daily_task_list(row)?;
            if let Some(tasks) = csv_to_vec(task_list) {
                curr_work.yesterday = Some(tasks);
            }
        }

    };

    Err("Today is beyond the range of the sheet".into())
}

fn get_daily_task_list(row: &Vec<serde_json::Value>) -> Result<String, Box<dyn std::error::Error>> {
    let mut task_list = "".to_string();
    for i in 8..=10 {
        let frag_task_list = serde_json::from_value::<String>(row[i].clone())?;
        task_list.push_str(frag_task_list.as_str());
    }

    Ok(task_list)
}

fn csv_to_vec(task_list: String) -> Option<Vec<String>> {
    if task_list.trim().is_empty() {
        return None
    }
    Some(task_list.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|task| return String::from(*task))
        .collect::<Vec<String>>())
}

fn get_day() -> u32 {
    let curr_time = chrono::offset::Local::now();
    curr_time.day()
}
