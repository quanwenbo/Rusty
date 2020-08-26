use std::thread;
use std::time::{Duration, SystemTime};

fn main() {


    let handle = thread::spawn(move || {
        for i in 1..=50 {
            println!("子线程在输出：{}", i);
            thread::sleep(Duration::from_secs(1));
        }
    });

    for n in 1..=50 {
        let time_first = SystemTime::now();
        println!("主线程在输出...{}", n);
        thread::sleep(Duration::from_secs_f64(2.5));

        match time_first.elapsed() {
            Ok(elapsed) => {
                // it prints '2'
                println!("主线程上一次执行{}秒后--->>", elapsed.as_secs());
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {:?}", e);
            }
        }

        // task01.join().unwrap();
    }

    handle.join().unwrap();
    // let x = 5;           // 值类型数据
    // let y = Box::new(x); // y 是一个智能指针，指向堆上存储的数据 5

    // println!("{}",5==x);
    // println!("{}",y);
    // println!("{:b}",*y);
    // println!("{:p}",&y);
    // println!("{:?}",&&&y);
}
