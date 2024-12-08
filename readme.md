## Usage

这是一个采用rust实现的服务请求 benchmark 工具，用于测试服务的qps和delay。

### TCP Server for test

在测试tcp server中，睡眠1s，测试qps。

```bash
cargo run --bin tcp_server
```

### Benchmark client

#### help info

```bash
cargo run --bin qps_client -- --help
```

```bash
qps_client 0.1.0

USAGE:
    qps_client [OPTIONS] [image_filename]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --batch-size <batch-size>             [default: 1]
        --connect-num <connect-num>           [default: 1]
        --consumer-num <consumer-num>         [default: 70]
        --measure-type <measure-type>         [default: quick]
        --model-name <model-name>             [default: resnet]
        --producer-num <producer-num>         [default: 3]
        --queue-capacity <queue-capacity>     [default: 1000]
        --url <url>                           [default: localhost:5006]

ARGS:
    <image_filename>
```

#### run 

```bash
bash run.sh
```

```bash
[INFO] Start 1 producers and 1 consumers
Producer: 0-10
Consumer: 1-1
connecting to server localhost:7878 times...
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
Received from server: Hello, server!
[STAT] Measure at stage 0, qps: 1, delay: 1.0022605606943928
```