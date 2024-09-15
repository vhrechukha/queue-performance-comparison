use beanstalkd::Beanstalkd;
use redis::{Client as RedisClient, Commands, ConnectionAddr, ConnectionInfo, ProtocolVersion};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <queue_type>", args[0]);
        return;
    }

    let queue_type = &args[1];

    match queue_type.as_str() {
        "beanstalkd" => {
            let mut client = Beanstalkd::connect("127.0.0.1", 11300).unwrap();
            let (job_id, job_body) = client.reserve().unwrap();
            println!("Message received from Beanstalkd: {:?}", job_body);
            client.delete(job_id).unwrap();
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
            let msg: String = con.rpop("queue", None).unwrap();
            println!("Message received from Redis (RDB): {}", msg);
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
            let msg: String = con.rpop("queue", None).unwrap();
            println!("Message received from Redis (AOF): {}", msg);
        }
        _ => {
            eprintln!("Unknown queue type: {}", queue_type);
        }
    }
}
