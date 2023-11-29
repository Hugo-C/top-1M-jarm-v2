mod queue;
mod storage;

use std::{env, error::Error, fs::File, thread};
use std::time::Duration;
use log::{debug, error, info};
use redis::{Connection, RedisResult};
use rust_jarm::Jarm;
use tempfile::NamedTempFile;
use crate::queue::{JarmResult, Task};

use std::io::Read;
use zip::read::ZipFile;


const JARM_HASH_FOR_DRY_RUN: &str = "27d27d27d0000001dc41d43d00041d1c5ac8aa552261ba8fd1aa9757c06fa5";
const TRANCO_DOWNLOAD_ZIP_URL: &str = "https://tranco-list.eu/top-1m.csv.zip";
const SAMPLE_TOP_1M_CSV_PATH: &str = "test/top-1m.csv";
const JARM_TIMEOUT: Duration = Duration::from_millis(2000);
const UPLOADER_POLL_INTERVAL: Duration = Duration::from_millis(1000);


pub fn run_scheduler(dry_run: bool) {
    let mut redis_connection = redis_connection().unwrap();
    schedule_tasks(&mut redis_connection, dry_run).expect("Failed to schedule all tasks");
}

pub fn run_worker(dry_run: bool) {
    let mut redis_connection = redis_connection().unwrap();
    process_tasks(&mut redis_connection, dry_run).expect("Failed to process all tasks");
}

pub fn run_uploader(dry_run: bool) {
    let mut redis_connection = redis_connection().unwrap();
    process_results(&mut redis_connection, dry_run).expect("Failed to upload all results");
}

fn redis_connection() -> RedisResult<Connection> {
    let redis_host = env::var("REDIS_HOST").unwrap_or("localhost".to_string());
    let redis_url = format!("redis://{redis_host}/");
    let client = redis::Client::open(redis_url)?;
    client.get_connection()
}

fn schedule_tasks(con: &mut Connection, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let content: String = if dry_run {
        let mut file_buf: Vec<u8> = Vec::new();
        let _ = File::open(SAMPLE_TOP_1M_CSV_PATH)?.read_to_end(&mut file_buf);
        String::from_utf8(file_buf)?
    } else {
        fetch_tranco_list()?
    };
    let mut reader = csv::ReaderBuilder::new();
    reader.has_headers(false);
    let mut rdr = reader.from_reader(content.as_bytes());
    for result in rdr.records() {
        let record = result?;
        debug!("{record:?} pushed to task queue");
        let rank = record.get(0).expect("rank is present");
        let domain = record.get(1).expect("domain is present");

        let task = Task { rank: rank.to_string(), domain: domain.to_string() };
        queue::push_task(con, task)?;
    }
    Ok(())
}

fn fetch_tranco_list() -> Result<String, Box<dyn Error>> {
    let mut res = reqwest::blocking::get(TRANCO_DOWNLOAD_ZIP_URL).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    let _ = res.read_to_end(&mut buf);

    let reader = std::io::Cursor::new(buf);
    let mut zip = zip::ZipArchive::new(reader).unwrap();
    let mut file_zip: ZipFile = zip.by_index(0)?;  // a single csv file is expected
    let mut file_buf: Vec<u8> = Vec::new();
    let _ = file_zip.read_to_end(&mut file_buf);
    let content = String::from_utf8(file_buf).unwrap();
    Ok(content)
}

fn process_tasks(con: &mut Connection, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let mut optionnal_task = queue::get_task(con);
    loop {
        match optionnal_task {
            None => break,
            Some(task) => {
                debug!("Processing: {}", task.domain);
                let jarm_hash = if dry_run {
                    Ok(JARM_HASH_FOR_DRY_RUN.to_string())
                } else {
                    let mut scanner = Jarm::new(task.domain.to_string(), 443.to_string());
                    scanner.timeout = JARM_TIMEOUT;
                    scanner.hash()
                };
                debug!("Jarm hash is {jarm_hash:?} for {}", task.domain);
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

fn process_results(con: &mut Connection, dry_run: bool) -> Result<(), Box<dyn Error>> {
    let bucket = if !dry_run {
        match storage::fetch_s3_bucket() {
            Ok(bucket) => {
                debug!("S3 credentials are valid");
                Some(bucket)
            }
            Err(e) => {
                error!("Error fetching the bucket from S3! {e}");
                error!("Invalid credentials ?");
                return Err(Box::try_from(e)?);
            }
        }
    } else { None };

    info!("Monitoring domains left to scan");
    let mut nb_tasks = queue::nb_tasks(con);
    let mut old_nb_task = 0;
    while nb_tasks > 0 {
        if nb_tasks != old_nb_task {
            debug!("{nb_tasks} domains left");
            old_nb_task = nb_tasks;
        }
        thread::sleep(UPLOADER_POLL_INTERVAL);
        nb_tasks = queue::nb_tasks(con);
    }
    info!("No more tasks left, all domains are scanned!");

    let tmp_file = NamedTempFile::new()?;
    let mut writer = csv::Writer::from_writer(&tmp_file);
    let mut optional_jarm_result = queue::get_jarm_result(con);
    loop {
        match optional_jarm_result {
            None => break,
            Some(jarm_result) => {
                let rank = jarm_result.rank;
                let domain = jarm_result.domain;
                let jarm_hash = jarm_result.jarm_hash;
                info!("Received: {rank} - {domain} - {jarm_hash}");
                writer.write_record(&[rank, domain, jarm_hash])?;  // TODO sort by rank
            }
        }
        optional_jarm_result = queue::get_jarm_result(con);
    }
    writer.flush()?;
    if !dry_run {
        let fetch_bucket = bucket.expect("bucket is present if not in dry run");
        storage::push_result_to_s3(fetch_bucket, &tmp_file).unwrap();
    }
    Ok(())
}