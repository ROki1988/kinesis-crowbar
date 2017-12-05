#[macro_use(lambda)]
extern crate crowbar;
#[macro_use]
extern crate cpython;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

lambda!(|event, context| {

    let xs: Vec<Record> = serde_json::from_value(event["Records"].clone())?;
    Ok(xs)
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
    approximateArrivalTimestamp: u32,
    partitionKey: String,
    data: String,
    kinesisSchemaVersion: String,
    sequenceNumber: String,
}
