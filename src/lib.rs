#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate data_encoding;

use data_encoding::BASE64;


lambda!(|event, context| {

    let xs: Vec<Record> = serde_json::from_value(event["Records"].clone())?;
    let h = xs.into_iter()
        .filter_map(|x| BASE64.decode(x.kinesis.data.as_bytes()).ok())
        .filter_map(|x| String::from_utf8(x).ok())
        .collect::<Vec<String>>();
    Ok(h)
});

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
