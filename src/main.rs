use lambda_runtime::{service_fn, LambdaEvent, Error};
use serde_json;

mod config;
mod sheets_reader;
mod http_client;
mod auth;
mod slack_messenger;

async fn handler(event: LambdaEvent<serde_json::Value>) -> Result<serde_json::Value, Error> {
    let payload = event.payload;
    Ok(serde_json::json!({ "message": "Hello, world!"}))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // lambda_runtime::run(service_fn(handler)).await
    let config = config::Config::init();
    let client = http_client::http_client();
    let auth = auth::auth(&config, client.clone()).await;
    let hub = google_sheets4::Sheets::new(client.clone(), auth);

    match sheets_reader::read(&config, &hub).await {
        Err(err) => {
            slack_messenger::send_message_err(&config, err.to_string()).await;

            return Ok(());
        },
        Ok((_, spreadsheet)) => {
            if let Some(vals) = spreadsheet.values {
                let curr_work = sheets_reader::get_current_work(vals);
                match curr_work {
                    Err(err) => slack_messenger::send_message_err(&config, err.to_string()).await,
                    Ok(work) => slack_messenger::send_message_ok(&config, &work).await
                };

                return Ok(());
            }

            slack_messenger::send_message_err(&config, "There was a problem connecting to the google sheets.  No values were found.".to_string()).await;
            
            return Ok(());
        }
    }
}
