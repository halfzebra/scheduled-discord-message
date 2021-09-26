use tokio_cron_async::JobSchedule;
use chrono::Local;
use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json;
use std::error::Error;
use std::process;
use std::env;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
struct DiscordWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
}

impl DiscordWebhook {
    pub fn new() -> Self {
        Self {
            username: None,
            content: None,
        }
    }

    pub fn username(&mut self, username: String) -> &mut Self {
        self.username = Some(username);
        self
    }

    pub fn content(&mut self, content: String) -> &mut Self {
        self.content = Some(content);
        self
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("start {}", Local::now().time());
    let sch: JobSchedule = JobSchedule::new();

    let webhook_url = if env::var("WEBHOOK_URL").is_ok() {
        Arc::new(env::var("WEBHOOK_URL").unwrap())
    } else {
        eprintln!("Missing WEBHOOK_URL env var");
        process::exit(1);
    };
    
    sch.add(
        "0 37 13 * * *",
        Box::new(move |_id| {
            let wh_clone = Arc::clone(&webhook_url);
            Box::pin(async move {
                let client = reqwest::Client::new();
                let mut message = DiscordWebhook::new();
            
                message
                    .username("Sally".to_owned())
                    .content("Hello".to_owned());

                let res = client
                    .post(wh_clone.as_str())
                    .body(
                        serde_json::to_string(
                            message
                                .username("My Bot".to_owned())
                                .content("Time for exercise!".to_owned()),
                        )
                        .unwrap(),
                    )
                    .header(CONTENT_TYPE, "application/json")
                    .send()
                    .await
                    .unwrap();
                
                if res.status().as_u16() == 204 {
                    println!("Message send successfully")
                } else {
                    eprintln!("Application error: {} {}", res.status().as_u16(), res.text().await.unwrap());
                }
            })
        }),
    )
    .await?;

    let schedule_runner = tokio::spawn(sch.run()).fuse();
    let interrupt = tokio::signal::ctrl_c().fuse();

    pin_mut!(schedule_runner, interrupt);

    select!(
        res = schedule_runner => match res {
            Ok(_jh) => {
                eprintln!("Runner returned");
                process::exit(1);
            },
            Err(err) => {
                eprintln!("Runner error: {}", err);
                process::exit(1);
            }
        },
        res = interrupt =>  match res {
            Ok(()) => {
                println!("Stopping process");
                process::exit(0);
            },
            Err(err) => {
                eprintln!("Application error: {}", err);
                process::exit(1);
            }
        },
    );
}
