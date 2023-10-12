use std::env;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
    let _ = dotenv::dotenv();
    let devs: Vec<String> = vec![env::var("KAFKA_BOOTSTRAP").unwrap().to_owned()];
    let topics = vec!["products"];
    for topic in topics {
        let mut consumer = Consumer::from_hosts(devs.to_owned())
            .with_topic(topic.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(Some(GroupOffsetStorage::Kafka))
            .create()
            .unwrap();
        while let Ok(it) = consumer.poll() {
            for ms in it.iter() {
                for m in ms.messages() {
                    println!("{:?}", m);
                }
            }
        }
    }
}
