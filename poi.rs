#![feature(duration_extras)]

use std::thread::*;
use std::time::*;
fn main() {
    fps_sleep()
}

fn fps_sleep() {
    let now = Instant::now();
    let mut start: Instant;
    let mut fps = 0.0;
    let mut count = 0;

    let mut fin = 0;

    const FPS: u32 = 60;

    start = Instant::now();
    for c in 0..301 {
        start = Instant::now();

        // ここに処理を書く

        while fin < 1000 {
            fin += 1;
        }

        let one_frame_sec: Duration = Duration::from_nanos(0_016_666_666);
        let sleep_time: Duration = one_frame_sec - start.elapsed();
        sleep(sleep_time);

        if count == FPS {
            let start_time = start.elapsed().subsec_nanos() as f64 * 1e-9;
            fps = 1.0 / start_time;
            println!("start_time poi: {}", start_time);
            count = 0;
        }

        count += 1;

        println!(
            "sleep time poi: {}",
            sleep_time.as_secs() as f64 + sleep_time.subsec_nanos() as f64 * 1e-9
        );
        println!("count poi: {}", count);
        println!("fps poi: {}", fps);
        println!("");
    }
}
