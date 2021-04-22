// extern crate tracing;
extern crate tokio;
use lapin::{
    message::DeliveryResult, options::*, publisher_confirm::Confirmation, types::FieldTable,
    BasicProperties, Connection, ConnectionProperties, Result,
};
use tokio_amqp::*;
// use tracing::*;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::{thread, time};


#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    let ten_millis = time::Duration::from_millis(1000);
    // tracing_subscriber::fmt::init();

    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(&addr, ConnectionProperties::default().with_tokio()).await?;

    info!("CONNECTED");

    let channel_a = conn.create_channel().await?;
    let channel_b = conn.create_channel().await?;

    let queue = channel_a
        .queue_declare(
            "hello",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    info!("Declared queue");

    // let consumer = channel_b
    //     .basic_consume(
    //         "hello",
    //         "my_consumer",
    //         BasicConsumeOptions::default(),
    //         FieldTable::default(),
    //     )
    //     .await?;

    // consumer.set_delegate(move |delivery: DeliveryResult| async move {
    //     // info!("delivery: {:?}", delivery.unwrap().delivery);

    //     let delivery = delivery.expect("error caught in in consumer");
    //     // info!("delivery: {:?}", delivery);
    //     if let Some(delivery) = delivery {
    //         // info!("delivery: {:?}", delivery);

    //         // delivery
    //         //     .acker(BasicAckOptions::default())
    //         //     .await
    //         //     .expect("failed to ack");
    //         (channel, delivery2 ) = delivery;
    //       info!("{:?}", delivery);
    //     }
    // });
    info!("{:?}",BasicConsumeOptions::default());
    let bcsoptions = BasicConsumeOptions {
        no_local: false,
        no_ack: true,
        exclusive: true,
        nowait: true,
    };
    // let channel = channel_b.clone();
    channel_b
        .basic_consume(
            "hello",
            "my_consumer",
            bcsoptions,
            FieldTable::default(),
        )
        .await
        .expect("basic_consume")
        .set_delegate(move |delivery: DeliveryResult| {
            // let channel = channel.clone();
            // let (_, delivery) = delivery;
            async move {
                // info!(message=?delivery, "received message");
                if let Ok(Some(delivery)) = delivery {
                    info!("delivery {:?}", delivery);
                    // delivery
                    //     .ack(BasicAckOptions::default())
                    //     .await
                    //     .expect("basic_ack");
                    let (_, delivery) = delivery;
                    info!("delivery2 {:?}", delivery.data);
                    // let msg = 
                    // delivery
                    //     .basic_get("hello", BasicGetOptions::default())
                    //     .await;
                    // info!("msg {:?}", msg);
                }
            }
        });
    let payload = b"Hello world!";

    loop {
        let confirm = channel_a
            .basic_publish(
                "",
                "hello",
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        thread::sleep(ten_millis);
    }
}
