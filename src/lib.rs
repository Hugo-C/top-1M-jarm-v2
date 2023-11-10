mod queue;

use std::{
    error::Error,
    fs::File,
};
use log::{info, trace};
use redis::{Connection, RedisResult};
use rust_jarm::Jarm;
use crate::queue::{JarmResult, Task};

const JARM_HASH_FOR_DRY_RUN: &str = "27d27d27d0000001dc41d43d00041d1c5ac8aa552261ba8fd1aa9757c06fa5";


pub fn run_scheduler() {
    let redis_connection = redis_connection().unwrap();
    schedule_tasks(redis_connection).expect("Failed to schedule all tasks");
}

pub fn run_worker(dry_run: bool) {
    let redis_connection = redis_connection().unwrap();
    process_tasks(redis_connection, dry_run).expect("Failed to process all tasks");
}

pub fn run_uploader() {
    let redis_connection = redis_connection().unwrap();
    process_results(redis_connection).expect("Failed to process all tasks");
}

fn redis_connection() -> RedisResult<Connection> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    client.get_connection()
}

fn schedule_tasks(mut con: Connection) -> Result<(), Box<dyn Error>> {
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
        queue::push_task(redis_connection().unwrap(), task)?;  // TODO replace by con
    }
    Ok(())
}

// TODO Uploader
// let mut queue_length: usize = con.llen(TASK_QUEUE)?;
// while queue_length > 0 {
//     let received: RedisResult<Vec<String>> = con.brpop(TASK_QUEUE, REDIS_BLOCK_TIMEOUT);
//     let vec_domain = match received {
//         Ok(vec_domain) => vec_domain,
//         Err(err) => return Err(Box::try_from(err).unwrap()),
//     };
//     let [_queue, domain] = &vec_domain[..] else { panic!("unexpected message format!") };
//     let jarm_hash = Jarm::new(domain.clone(), 443.to_string()).hash();
//     println!("RECEIVED: {domain:?}, jarm {jarm_hash:?}");
//     if let Ok(hash) = jarm_hash {
//         let _: () = con.lpush(RESULT_QUEUE, hash)?;
//     }
//     queue_length = con.llen(TASK_QUEUE)?;
// }
fn process_tasks(mut con: Connection, dry_run: bool) -> Result<(), Box<dyn Error>> {
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
                        jarm_hash: hash
                    };
                    queue::push_jarm_result(redis_connection().unwrap(), jarm_result)?;  // TODO replace by con
                }
            }
        }
        optionnal_task = queue::get_task(redis_connection().unwrap());  // TODO replace by con
    }
    Ok(())
}

fn process_results(mut con: Connection) -> Result<(), Box<dyn Error>> {
    let mut optionnal_jarm_result = queue::get_jarm_result(con);
    loop {
        match optionnal_jarm_result {
            None => break,
            Some(jarm_result) => {
                let rank = jarm_result.rank;
                let domain = jarm_result.domain;
                let jarm_hash = jarm_result.jarm_hash;
                info!("Received: {rank} - {domain} - {jarm_hash}");
            }
        }
        optionnal_jarm_result = queue::get_jarm_result(redis_connection().unwrap()); // TODO replace by con
    }
    Ok(())
}