use beanstalkd::Beanstalkd;
use redis::{Client as RedisClient, Commands, ConnectionAddr, ConnectionInfo, ProtocolVersion};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <queue_type> <message>", args[0]);
        return;
    }

    let queue_type = &args[1];
    let message = &args[2];

    match queue_type.as_str() {
        "beanstalkd" => {
            let mut beanstalkd = Beanstalkd::connect("127.0.0.1", 11300).unwrap();
            beanstalkd.put(message, 10, 0, 120).unwrap();
            println!("Message sent to Beanstalkd");
        }
        "redis_rdb" => {
            let client = RedisClient::open(ConnectionInfo {
                addr: *Box::new(ConnectionAddr::Tcp("127.0.0.1".to_string(), 6379)),
                redis: redis::RedisConnectionInfo {
                    db: 0,
                    username: None,
                    password: None,
                    protocol: ProtocolVersion::RESP2,
                },
            })
            .unwrap();
            let mut con = client.get_connection().unwrap();
            let _: () = con.lpush("queue", message).unwrap();
            println!("Message sent to Redis (RDB)");
        }
        "redis_aof" => {
            let client = RedisClient::open(ConnectionInfo {
                addr: *Box::new(ConnectionAddr::Tcp("127.0.0.1".to_string(), 6380)),
                redis: redis::RedisConnectionInfo {
                    db: 0,
                    username: None,
                    password: None,
                    protocol: ProtocolVersion::RESP2,
                },
            })
            .unwrap();
            let mut con = client.get_connection().unwrap();
            let _: () = con.lpush("queue", message).unwrap();
            println!("Message sent to Redis (AOF)");
        }
        _ => {
            eprintln!("Unknown queue type: {}", queue_type);
        }
    }
}
