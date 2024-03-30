use std::thread;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, Exchange, ExchangeDeclareOptions, ExchangeType, Publish, QueueDeclareOptions};
use amiquip::AmqpValue::LongString;

fn main() -> amiquip::Result<()>  {
    // Dead Letter Queue
    let mut connection =
        Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    exchange.publish(Publish::new(b"Hello World from customer", "primary_queue"))?;

    let primary_handler = thread::spawn(move || {
        if let Err(err) = consume_primary_queue() {
            eprintln!("Error in consuming from primary queue: {}", err);
        }
    });

    let dlq_handler = thread::spawn(move || {
        if let Err(err) = dlq_consume() {
            eprintln!("Error in consuming from dlq: {}", err);
        }
    });

    primary_handler.join().expect("Primary thread panicked");
    dlq_handler.join().expect("Primary thread panicked");
    connection.close()
}

fn consume_primary_queue() -> amiquip::Result<()> {
    let mut connection =
        Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let mut queue_options = QueueDeclareOptions::default();
    queue_options.arguments.insert("x-dead-letter-exchange".to_string(),
    LongString("dead_letter_exchange".to_string()));

    queue_options.arguments.insert("x-dead-letter-routing-key".to_string(),
    LongString("dead_letter_routing_key".to_string()));

    let primary_queue =
        channel.queue_declare("primary_queue", queue_options)?;

    let consumer = primary_queue.consume(ConsumerOptions::default())?;

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                println!("Received message number {} in primary_queue", i);
                consumer.reject(delivery, false)?;
                println!("Message Rejected in Primary Queue");
            },
            other  => {
                println!("Consumer ended {:?}", other);
                break;
            }
        }
    }
    connection.close()
}

fn dlq_consume() -> amiquip::Result<()> {
    let mut connection =
        Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    let channel = connection.open_channel(None)?;

    let dlx = channel.exchange_declare(
        ExchangeType::Direct,
        "dead_letter_exchange",
        ExchangeDeclareOptions::default(),
    )?;

    let dlq = channel.queue_declare("dead_letter_queue",
                                    QueueDeclareOptions::default())?;

    dlq.bind(&dlx, "dead_letter_routing_key", Default::default())?;

    let consumer = dlq.consume(ConsumerOptions::default())?;

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                println!("Received message number {} in dead_letter_queue with body {}", i, body);
                consumer.ack(delivery)?;
            },
            other  => {
                println!("Consumer ended {:?}", other);
                break;
            }
        }
    }
    connection.close()
}
