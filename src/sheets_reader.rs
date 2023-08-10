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
        .values_get(&config.sheets_id, &config.deposit_range_input)
        .doit()
        .await
}

pub fn get_current_work(data: Vec<Vec<serde_json::Value>>) -> Result<CurrentWork, Box<dyn std::error::Error>> {
    let today = get_day();
    let yesterday = if today > 0 { today - 1 } else { 0 };

    let mut curr_work = CurrentWork {
        yesterday: None,
        today: None
    };

    for row in data.iter() {
        let day_num = serde_json::from_value::<u32>(row[2].clone())?;
        if day_num == today {
            let task_list = serde_json::from_value::<String>(row[7].clone())?;
            let task_list = csv_to_vec(task_list);

            curr_work.today = Some(task_list);

            return Ok(curr_work);
        }

        if day_num == yesterday {
            let task_list = serde_json::from_value::<String>(row[7].clone())?;
            let task_list = csv_to_vec(task_list);

            curr_work.yesterday = Some(task_list);
        }

    };

    Err("Today is beyond the range of the sheet".into())
}

fn csv_to_vec(task_list: String) -> Vec<String> {
    task_list.split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|task| return String::from(*task))
        .collect::<Vec<String>>()
}

fn get_day() -> u32 {
    let curr_time = chrono::offset::Local::now();
    curr_time.day0()
}
