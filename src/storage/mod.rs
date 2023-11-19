use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Read;
use log::debug;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::error::S3Error;
use s3::Region;
use tempfile::NamedTempFile;

const S3_ENDPOINT: &str = "https://storage.googleapis.com";
const S3_BUCKET_NAME: &str = "tranco-jarm";
const S3_BUCKET_REGION_NAME: &str = "us-central1";
const S3_PATH: &str = "/jarm-tranco-top-1m.csv";

pub(crate) fn fetch_s3_bucket() -> Result<Bucket, S3Error> {
    let region_name = S3_BUCKET_REGION_NAME.to_string();
    let region = Region::Custom { region: region_name, endpoint: S3_ENDPOINT.to_string() };

    let credentials = Credentials::from_env()?;
    let bucket = Bucket::new(S3_BUCKET_NAME, region, credentials)?;

    // We check we have the correct credentials, head should return HTTP 403 if so
    let head = bucket.head_object_blocking(S3_PATH);
    match head {
        Ok(_) => Ok(bucket),
        Err(s3_error) => match s3_error {
            S3Error::Http(404, _) => Ok(bucket),  // File simply do not exists, we can continue
            _ => Err(s3_error)
        }
    }
}

pub(crate) fn push_result_to_s3(bucket: Bucket, tmp_file: &NamedTempFile) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(tmp_file.path())?;
    let metadata = fs::metadata(tmp_file.path())?;
    let mut content = vec![0; metadata.len() as usize];
    file.read_exact(&mut content).unwrap();

    let result = bucket.put_object_with_content_type_blocking(S3_PATH, &content, "text/csv")?;
    debug!("S3 response {}", result.status_code());
    Ok(())
}
