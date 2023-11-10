mod queue;

use std::{error::Error, fs::File, thread};
use std::time::Duration;
use log::{info, trace};
use redis::{Connection, RedisResult};
use rust_jarm::Jarm;
use crate::queue::{JarmResult, Task};

const JARM_HASH_FOR_DRY_RUN: &str = "27d27d27d0000001dc41d43d00041d1c5ac8aa552261ba8fd1aa9757c06fa5";
const UPLOADER_POLL_INTERVAL: Duration = Duration::from_millis(100);


pub fn run_scheduler() {
    let mut redis_connection = redis_connection().unwrap();
    schedule_tasks(&mut redis_connection).expect("Failed to schedule all tasks");
}

pub fn run_worker(dry_run: bool) {
    let mut redis_connection = redis_connection().unwrap();
    process_tasks(&mut redis_connection, dry_run).expect("Failed to process all tasks");
}

pub fn run_uploader() {
    let mut redis_connection = redis_connection().unwrap();
    process_results(&mut redis_connection).expect("Failed to upload all results");
}

fn redis_connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

fn schedule_tasks(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    let file = File::open("top-1m.csv")?;
    let mut reader = csv::ReaderBuilder::new();
    reader.has_headers(false);
    let mut rdr = reader.from_reader(file);
    for result in rdr.records() {
        let record = result?;
        trace!("{record:?} pushed to task queue");
        let rank = record.get(0).expect("rank is present");
        let domain = record.get(1).expect("domain is present");

        let task = Task { rank: rank.to_string(), domain: domain.to_string() };
        queue::push_task(con, task)?;
    }
    Ok(())
}

fn process_tasks(con: &mut Connection, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let mut optionnal_task = queue::get_task(con);
    loop {
        match optionnal_task {
            None => break,
            Some(task) => {
                trace!("Processing: {}", task.domain);
                let jarm_hash = if dry_run {
                    Ok(JARM_HASH_FOR_DRY_RUN.to_string())
                } else {
                    Jarm::new(task.domain.to_string(), 443.to_string()).hash()
                };
                trace!("Jarm hash is {jarm_hash:?} for {}", task.domain);
                if let Ok(hash) = jarm_hash {
                    let jarm_result = JarmResult {
                        rank: task.rank,
                        domain: task.domain,
                        jarm_hash: hash,
                    };
                    queue::push_jarm_result(con, jarm_result)?;
                }
            }
        }
        optionnal_task = queue::get_task(con);
    }
    Ok(())
}

fn process_results(con: &mut Connection) -> Result<(), Box<dyn Error>> {
    info!("Monitoring domains left to scan");
    let mut nb_tasks = queue::nb_tasks(con);
    let mut old_nb_task = 0;
    while nb_tasks > 0 {
        if nb_tasks != old_nb_task {
            trace!("{nb_tasks} domains left");
            old_nb_task = nb_tasks;
        }
        thread::sleep(UPLOADER_POLL_INTERVAL);
        nb_tasks = queue::nb_tasks(con);
    }
    info!("No more tasks left, all domains are scanned!");

    let mut optional_jarm_result = queue::get_jarm_result(con);
    loop {
        match optional_jarm_result {
            None => break,
            Some(jarm_result) => {
                let rank = jarm_result.rank;
                let domain = jarm_result.domain;
                let jarm_hash = jarm_result.jarm_hash;
                info!("Received: {rank} - {domain} - {jarm_hash}");
            }
        }
        optional_jarm_result = queue::get_jarm_result(con);
    }
    Ok(())
}