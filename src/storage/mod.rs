use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use log::debug;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region;
use tempfile::NamedTempFile;

const S3_ENDPOINT: &str = "https://storage.googleapis.com";

// TODO add credential test

pub(crate) fn push_result_to_s3(tmp_file: &NamedTempFile) -> Result<(), Box<dyn Error>> {
    let region_name = "us-central1".to_string();
    let region = Region::Custom { region: region_name, endpoint: S3_ENDPOINT.to_string() };

    let credentials = Credentials::from_env()?;
    let bucket = Bucket::new("tranco-jarm", region, credentials)?;
    let mut file = File::open(tmp_file.path())?;
    let metadata = fs::metadata(tmp_file.path())?;
    let mut content = vec![0; metadata.len() as usize];
    file.read_exact(&mut content).unwrap();

    let result = bucket.put_object_with_content_type_blocking("/jarm-tranco-top-1m.csv", &content, "text/csv")?;
    debug!("S3 response {}", result.status_code());
    Ok(())
}
