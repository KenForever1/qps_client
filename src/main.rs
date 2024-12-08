use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam_channel::{unbounded, Sender, Receiver};
use structopt::StructOpt;

 mod infer_client;
 mod tcp_about;

use crate::tcp_about::client::TcpClient;
use crate::infer_client::InferClient;


#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long, default_value = "resnet")]
    model_name: String,
    #[structopt(long, default_value = "1")]
    batch_size: usize,
    #[structopt(long, default_value = "localhost:5006")]
    url: String,
    #[structopt(long, default_value = "1")]
    connect_num: usize,
    #[structopt(long, default_value = "1000")]
    queue_capacity: usize,
    #[structopt(long, default_value = "3")]
    producer_num: usize,
    #[structopt(long, default_value = "70")]
    consumer_num: usize,
    #[structopt(long, default_value = "quick")]
    measure_type: String,
    #[structopt(name = "image_filename")]
    image_filename: Option<String>,
}

fn measure(
    request_sum: Arc<Mutex<usize>>,
    delay_sum: Arc<Mutex<f64>>,
    producer_stop_flag: Arc<Mutex<bool>>,
    measure_type: &str,
    batch_size: usize,
) {
    let measure_times = match measure_type {
        "no" => 0,
        "quick" => 10,
        "long" => 1_000_000_000,
        _ => 0,
    };

    for idx in 0..measure_times {
        thread::sleep(Duration::from_secs(1));


        let sum_stage1 = *request_sum.lock().expect("Failed to lock request_sum");
        // set delay_sum to 0
        *delay_sum.lock().expect("Failed to lock delay_sum") = 0.0;

        thread::sleep(Duration::from_secs(10));

        let (sum_stage2, delay_cur) = {
            let req = request_sum.lock().expect("Failed to lock request_sum");
            let del = delay_sum.lock().expect("Failed to lock delay_sum");
            (*req, *del)
        };

        let sum_diff = sum_stage2 - sum_stage1;
        let ave_delay = delay_cur / (sum_diff as f64 + 0.0001) * batch_size as f64;
        println!(
            "[STAT] Measure at stage {}, qps: {}, delay: {}",
            idx,
            sum_diff as f64 / 10.0,
            ave_delay
        );
    }

    let mut stop_flag = producer_stop_flag.lock().expect("Failed to lock producer_stop_flag");
    *stop_flag = true;
}

fn producer(
    sender: Sender<Vec<String>>,
    img_list: Vec<String>,
    idx_start: usize,
    idx_end: usize,
    measure_type: &str,
    stop_flag: Arc<Mutex<bool>>,
    batch_size: usize,
) {
    println!("Producer: {}-{}", idx_start, idx_end);
    if measure_type == "no" {
        for idx in (idx_start..idx_end).step_by(batch_size) {
            let batch = img_list[idx..std::cmp::min(idx + batch_size, idx_end)].to_vec();
            if sender.send(batch).is_err() {
                eprintln!("Failed to send batch");
                break;
            }

            if *stop_flag.lock().expect("Failed to lock stop_flag") {
                break;
            }
        }
    } else if measure_type == "quick" || measure_type == "long" {
        loop {
            if *stop_flag.lock().expect("Failed to lock stop_flag") {
                break;
            }

            for idx in (idx_start..idx_end).step_by(batch_size) {
                if *stop_flag.lock().expect("Failed to lock stop_flag") {
                    break;
                }
                let batch = img_list[idx..std::cmp::min(idx + batch_size, idx_end)].to_vec();
                if sender.send(batch).is_err() {
                    eprintln!("Failed to send batch");
                    break;
                }
            }
        }
    }
}

fn consumer(
    receiver: Receiver<Vec<String>>,
    request_sum: Arc<Mutex<usize>>,
    delay_sum: Arc<Mutex<f64>>,
    url: String,
    batch_size: usize,
    connect_num: usize,
) {
    println!("Consumer: {}-{}", connect_num, batch_size);
    let mut client = TcpClient::new(url);
    println!("connecting to server {} times...", client.get_url()
    );

    while let Ok(path_list) = receiver.recv() {
        let start = Instant::now();
        // Simulate infer
        // thread::sleep(Duration::from_millis(100)); // Placeholder for actual inference logic
        let _ = client.infer();
        
        let duration = start.elapsed();

        if let Ok(mut delay) = delay_sum.lock() {
            *delay += duration.as_secs_f64();
        } else {
            eprintln!("Failed to lock delay_sum");
        }

        if let Ok(mut req) = request_sum.lock() {
            *req += batch_size;
        } else {
            eprintln!("Failed to lock request_sum");
        }
    }
}

fn collect_imgs(image_filename: &Option<String>) -> Vec<String> {
    if let Some(path) = image_filename {
        // Placeholder to simulate image collection
        vec![path.clone(); 10] // Simulate 10 images
    } else {
        vec!["hello world".to_string(); 10] // Simulate 10 images
    }
}

fn main() {
    let opt = Opt::from_args();

    let img_list = collect_imgs(&opt.image_filename);
    let img_list_size = img_list.len();
    let producer_num = std::cmp::min(opt.producer_num, img_list_size);
    let block_size = img_list_size / producer_num;

    let (sender, receiver) = unbounded();
    let request_sum = Arc::new(Mutex::new(0));
    let delay_sum = Arc::new(Mutex::new(0.0));
    let producer_stop_flag = Arc::new(Mutex::new(false));

    let mut producers = vec![];
    let mut consumers = vec![];

    println!("[INFO] Start {} producers and {} consumers", producer_num, opt.consumer_num);
    for producer_idx in 0..producer_num {
        let idx_start = producer_idx * block_size;
        let idx_end = if producer_idx == producer_num - 1 {
            img_list_size
        } else {
            (producer_idx + 1) * block_size
        };

        let tx_clone = sender.clone();
        let img_list_clone = img_list.clone();
        let stop_flag = Arc::clone(&producer_stop_flag);
        let measure_type = opt.measure_type.clone();

        producers.push(thread::spawn(move || {
            producer(
                tx_clone,
                img_list_clone,
                idx_start,
                idx_end,
                &measure_type,
                stop_flag,
                opt.batch_size,
            );
        }));
    }

    for _ in 0..opt.consumer_num {
        let request_sum_clone = Arc::clone(&request_sum);
        let delay_sum_clone = Arc::clone(&delay_sum);
        let url = opt.url.clone();
        let rx_clone = receiver.clone();

        consumers.push(thread::spawn(move || {
            consumer(
                rx_clone,
                request_sum_clone,
                delay_sum_clone,
                url,
                opt.batch_size,
                opt.connect_num,
            );
        }));
    }

    let request_sum_clone = Arc::clone(&request_sum);
    let delay_sum_clone = Arc::clone(&delay_sum);
    let producer_stop_flag_clone = Arc::clone(&producer_stop_flag);
    let measure_type = opt.measure_type.clone();

    let measure_thread = thread::spawn(move || {
        measure(
            request_sum_clone,
            delay_sum_clone,
            producer_stop_flag_clone,
            &measure_type,
            opt.batch_size,
        );
    });

    for producer in producers {
        if let Err(e) = producer.join() {
            eprintln!("Producer thread failed: {:?}", e);
        }
    }

    for consumer in consumers {
        if let Err(e) = consumer.join() {
            eprintln!("Comsumer thread failed: {:?}", e);
        }
    }

    if let Err(e) = measure_thread.join() {
        eprintln!("Measure thread failed: {:?}", e);
    }

    println!("Done!");
}