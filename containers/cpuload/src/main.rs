use rand::Rng;
use std::env;
use std::thread;
use std::time::Duration;
use std::time::Instant;

fn hog_cpu(n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let y: f64 = rng.gen();
        let _ = y.sqrt();
    }
}

fn calc_hog_iterations(d: Duration) -> u32 {
    let ratio = 1000;
    let mut n = 0;

    let start = Instant::now();
    loop {
        hog_cpu(ratio);
        if start.elapsed() > d {
            break;
        }
        n += 1;
    }
    return n * ratio;
}

fn main() {
    let hog_time = match env::var("HOG_SECONDS") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_e) => 1,
    };
    let idle_time = match env::var("IDLE_SECONDS") {
        Ok(val) => val.parse::<u64>().unwrap(),
        Err(_e) => 9,
    };

    let idle = Duration::from_secs(idle_time);
    let n = calc_hog_iterations(Duration::from_secs(hog_time));

    loop {
        let start = Instant::now();
        hog_cpu(n);
        let duration = start.elapsed();
        println!("CPU hog: {:?}", duration);

        println!("Idle time: {:?}", idle);
        thread::sleep(idle);
    }
}
