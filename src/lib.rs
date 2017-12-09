#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate data_encoding;
extern crate rayon;

extern crate rusoto_core;
extern crate rusoto_s3;

use rusoto_s3::{S3, S3Client, PutObjectRequest};
use rusoto_core::DefaultCredentialsProvider;
use rusoto_core::region::default_region;
use rusoto_core::default_tls_client;

use std::env;

use data_encoding::BASE64;
use rayon::prelude::*;
use crowbar::{Value, LambdaContext, LambdaResult};

fn my_handler(event: Value, context: LambdaContext) -> LambdaResult {
    let provider = DefaultCredentialsProvider::new()?;
    let tls = default_tls_client()?;
    let client = S3Client::new(tls, provider, default_region());

    let xs: Vec<Record> = serde_json::from_value(event["Records"].clone())?;
    let h = xs.into_par_iter()
        .filter_map(|x| BASE64.decode(x.kinesis.data.as_bytes()).ok())
        .filter_map(|x| String::from_utf8(x).ok())
        .collect::<Vec<String>>();

    let bucket = env::var("TARGET_BUCKET")?;
    let request = PutObjectRequest {
        bucket,
        key: "sample".to_owned(),
        body: serde_json::to_vec(&h).ok(),
        ..Default::default()
    };
    let out = client.put_object(&request)?;
    Ok(serde_json::Value::from("success".to_owned()))
}

lambda!(my_handler);

#[derive(Serialize, Deserialize, Debug)]
struct Record {
    eventID: String,
    eventVersion: String,
    kinesis: Kinesis,
    invokeIdentityArn: String,
    eventName: String,
    eventSourceARN: String,
    eventSource: String,
    awsRegion: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Kinesis {
    approximateArrivalTimestamp: f64,
    partitionKey: String,
    data: String,
    kinesisSchemaVersion: String,
    sequenceNumber: String,
}
