use std::error::Error;
use std::fs::File;
use std::io::{stderr, stdin, BufRead, BufReader, Write};
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use kafka::client::{Compression, KafkaClient, RequiredAcks};
use kafka::producer::{AsBytes, Producer, Record};

struct Config {
    brokers: Vec<String>,
    topic: String,
    input_file: Option<String>,
    compression: Compression,
    required_acks: RequiredAcks,
    batch_size: usize,
    conn_idle_timeout: Duration,
    ack_timeout: Duration,
}
struct Trimmed(String);

impl AsBytes for Trimmed {
    fn as_bytes(&self) -> &[u8] {
        self.0.trim().as_bytes()
    }
}

impl Deref for Trimmed {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Trimmed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn produce(cfg: &Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut client = KafkaClient::new(cfg.brokers.clone());
    client.set_client_id("kafka-rust-console-producer".into());
    client.load_metadata_all()?;

    // ~ verify that the remote brokers do know about the target topic
    //ensure!(client.topics().contains(&cfg.topic));
    match cfg.input_file {
        None => {
            let stdin = stdin();
            let mut stdin = stdin.lock();
            produce_impl(&mut stdin, client, cfg)
        }
        Some(ref file) => {
            let mut r = BufReader::new(File::open(file)?);
            produce_impl(&mut r, client, cfg)
        }
    }
}

fn produce_impl(src: &mut dyn BufRead, client: KafkaClient, cfg: &Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut producer = Producer::from_client(client)
        .with_ack_timeout(cfg.ack_timeout)
        .with_required_acks(cfg.required_acks)
        .with_compression(cfg.compression)
        .with_connection_idle_timeout(cfg.conn_idle_timeout)
        .create()?;
    produce_impl_nobatch(&mut producer, src, cfg)
}

fn produce_impl_nobatch(producer: &mut Producer, src: &mut dyn BufRead, cfg: &Config) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut stderr = stderr();
    let mut rec = Record::from_value(&cfg.topic, Trimmed(String::new()));
    loop {
        rec.value.clear();
        if src.read_line(&mut rec.value)? == 0 {
            break; // ~ EOF reached
        }
        if rec.value.trim().is_empty() {
            continue; // ~ skip empty lines
        }
        // ~ directly send to kafka
        producer.send(&rec)?;
        let _ = write!(stderr, "Sent: {}", *rec.value);
    }
    Ok(())
}
