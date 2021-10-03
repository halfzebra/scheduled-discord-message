use futures::{
    future::FutureExt, // for `.fuse()`
    pin_mut,
    select,
};
use reqwest::header::CONTENT_TYPE;
use serde_json;
use tokio::fs;
use tokio_cron_async::JobSchedule;
use structopt::StructOpt;
use scheduled_discord_message::{CronDiscordWebhook};

use std::path::PathBuf;
use std::error::Error;
use std::process;

#[derive(StructOpt, Debug)]
#[structopt(name = "scheduled-discord-message")]
struct Opt {
    /// Output file
    #[structopt(short = "c", long, parse(from_os_str))]
    config: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let sch: JobSchedule = JobSchedule::new();
    let contents = fs::read_to_string(opt.config).await?;
    let cron_webhooks: Vec<CronDiscordWebhook> = serde_json::from_str(contents.as_str())?;

    for wh in cron_webhooks {
        sch.add(
            wh.schedule.clone(),
            Box::new(move |_id| {
                let wh = wh.clone();
                Box::pin(async move {
                    let client = reqwest::Client::new();
                    let res = client
                        .post(wh.url.as_str())
                        .body(serde_json::to_string(&wh.webhook).unwrap())
                        .header(CONTENT_TYPE, "application/json")
                        .send()
                        .await
                        .unwrap();
                    if res.status().as_u16() == 204 {
                        println!("Message send successfully")
                    } else {
                        eprintln!(
                            "Application error: {} {}",
                            res.status().as_u16(),
                            res.text().await.unwrap()
                        );
                    }
                })
            }),
        )
        .await?;
    }

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
