use beanstalkd::Beanstalkd;
use redis::{
    Client as RedisClient, Commands, ConnectionAddr, ConnectionInfo, ProtocolVersion,
    RedisConnectionInfo,
};
use std::time::Instant;

fn benchmark_beanstalkd(num_operations: u64) {
    let mut client = Beanstalkd::connect("127.0.0.1", 11300).unwrap();

    let start = Instant::now();

    for i in 0..num_operations {
        let message = format!("message {}", i);
        client.put(&message, 10, 0, 120).unwrap();
    }

    let duration = start.elapsed();
    let ops = num_operations as f64 / duration.as_secs_f64();

    println!("Beanstalkd: {} ops/sec", ops);
}

fn benchmark_redis(port: u16, description: &str, num_operations: u64) {
    let client = RedisClient::open(ConnectionInfo {
        addr: *Box::new(ConnectionAddr::Tcp("127.0.0.1".to_string(), port)),
        redis: RedisConnectionInfo {
            db: 0,
            username: None,
            password: None,
            protocol: ProtocolVersion::RESP2,
        },
    })
    .unwrap();
    let mut con = client.get_connection().unwrap();

    let start = Instant::now();

    for i in 0..num_operations {
        let _: () = con.lpush("queue", format!("message {}", i)).unwrap();
    }

    let duration = start.elapsed();
    let ops = num_operations as f64 / duration.as_secs_f64();

    println!("{}: {} ops/sec", description, ops);
}

fn main() {
    let num_operations = 1_000_000;

    println!("Benchmarking Beanstalkd with {} operations...", num_operations);
    benchmark_beanstalkd(num_operations);

    println!("Benchmarking Redis with RDB...");
    benchmark_redis(6379, "Redis RDB", num_operations);

    println!("Benchmarking Redis with AOF...");
    benchmark_redis(6380, "Redis AOF", num_operations);
}
