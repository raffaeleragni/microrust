use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

fn main() {
    let devs: Vec<String> = vec!["localhost:9094".to_owned()];
    let topics = vec!["events"];
    for topic in topics {
        let mut consumer = Consumer::from_hosts(devs.to_owned())
            .with_topic(topic.to_owned())
            .with_fallback_offset(FetchOffset::Earliest)
            .with_offset_storage(GroupOffsetStorage::Kafka)
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
