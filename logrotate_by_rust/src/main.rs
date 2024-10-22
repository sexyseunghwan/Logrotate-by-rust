// use std::fs::{File, rename};
// use std::io::{Write};
// use std::path::Path;
// use fs2::FileExt; // 파일 락을 위한 라이브러리
// use std::thread;
// use std::time::{Duration, SystemTime};

// fn rotate_log(log_file_path: &str) -> std::io::Result<()> {
//     let date = Local::now();
//     let new_log_file = format!("vector-{}-{:02}-{:02}.log", date.year(), date.month(), date.day());

//     // 현재 로그 파일을 잠금 (쓰기 중 이동 방지)
//     let file = File::open(log_file_path)?;
//     file.lock_exclusive()?; // 파일을 배타적으로 잠금

//     // 로그 파일을 날짜별로 이동
//     rename(log_file_path, new_log_file)?;

//     // 잠금 해제
//     file.unlock()?;

//     // 새로운 빈 로그 파일 생성
//     let mut new_file = File::create(log_file_path)?;
//     new_file.write_all(b"")?; // 빈 파일 생성

//     Ok(())
// }

mod handler;
mod service;
mod utils;
mod model;

mod common;
use common::*;

fn main() {
    
    //let log_file_path = "C:\\Users\\user\\Desktop\\vector\\logs\\vector.log";
    
    // loop {
    //     // 현재 시간을 가져옴
    //     let now = Local::now();
    //     let next_midnight = (now + chrono::Duration::days(1)).date().and_hms(0, 0, 0);
    //     let duration_until_midnight = (next_midnight - now).to_std().unwrap();

    //     // 자정까지 대기
    //     thread::sleep(duration_until_midnight);

    //     // 로그 회전 수행
    //     if let Err(e) = rotate_log(log_file_path) {
    //         eprintln!("로그 회전 중 오류 발생: {:?}", e);
    //     }

    //     // 자정 이후 계속 대기
    //     thread::sleep(Duration::from_secs(60));
    // }
}