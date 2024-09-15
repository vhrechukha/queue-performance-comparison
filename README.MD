
# Queue Performance Comparison

This project benchmarks the performance of Beanstalkd and Redis (with RDB and AOF persistence) queueing systems.

### View Benchmarks

1. Start the Docker containers for Beanstalkd and Redis (RDB and AOF):

   ```bash
   docker-compose up -d
   ```

   This command starts the following services:
   - **Beanstalkd** on port `11300`
   - **Redis with RDB** on port `6379`
   - **Redis with AOF** on port `6380`

2. Navigate to the `app` directory:

   ```bash
   cd app
   ```

3. Run the Rust benchmarking script:

   ```bash
   cargo run --bin benchmark
   ```

   This will execute the benchmarks and output the operations per second (OPS) for Beanstalkd, Redis with RDB, and Redis with AOF.

4. The result will be shown at the end of the operation.

### Manually Put and Read Messages in Queues

1. Start the Docker containers for Beanstalkd and Redis (RDB and AOF):

   ```bash
   docker-compose up -d
   ```

   This command starts the following services:
   - **Beanstalkd** on port `11300`
   - **Redis with RDB** on port `6379`
   - **Redis with AOF** on port `6380`

2. Navigate to the `app` directory:

   ```bash
   cd app
   ```

### 3. Put a Message into the Queue

You can separately put a message into the queue using the following commands:

1. **Put a message into Beanstalkd**:

   ```bash
   cargo run --bin put_message beanstalkd "Hello Beanstalkd!"
   ```

2. **Put a message into Redis with RDB**:

   ```bash
   cargo run --bin put_message redis_rdb "Hello Redis RDB!"
   ```

3. **Put a message into Redis with AOF**:

   ```bash
   cargo run --bin put_message redis_aof "Hello Redis AOF!"
   ```

### 4. Read a Message from the Queue

To read a message from the queue, use the following commands:

1. **Read a message from Beanstalkd**:

   ```bash
   cargo run --bin read_message beanstalkd
   ```

2. **Read a message from Redis with RDB**:

   ```bash
   cargo run --bin read_message redis_rdb
   ```

3. **Read a message from Redis with AOF**:

   ```bash
   cargo run --bin read_message redis_aof
   ```

## Benchmark Results

Here are the benchmark results after running the tests:

```bash
% cargo run --bin benchmark
Benchmarking Beanstalkd with 100000 operations...
Beanstalkd: 9818.337759841403 ops/sec
Benchmarking Redis with RDB...
Redis RDB: 9367.447788246203 ops/sec
Benchmarking Redis with AOF...
Redis AOF: 6982.478891900849 ops/sec
```

```bash
% cargo run --bin benchmark
Benchmarking Beanstalkd with 1000000 operations...
Beanstalkd: 9824.590902625478 ops/sec
Benchmarking Redis with RDB...
Redis RDB: 9970.978022673196 ops/sec
Benchmarking Redis with AOF...
Redis AOF: 6607.912056878202 ops/sec
```
