use crate::common::*;

use crate::service::comprs_service::*;
use crate::service::file_service::*;

use crate::utils::io_utils::*;

use crate::model::Settings::*;

pub struct LogrotateHandler<C: ComrsService, F: FileService> {
    comprs_service: C,
    file_service: F
}


impl<C: ComrsService, F: FileService> LogrotateHandler<C,F> {   
    
    pub fn new(comprs_service: C, file_service: F) -> Self {
        Self{ comprs_service, file_service }
    }     

    
    #[doc = "로그 로테이팅을 수행해주는 메인 핸들러 함수"]
    pub fn log_rotate_main_handler() -> Result<(), anyhow::Error> {
        
        let settings: Settings = read_yml_from_file("./config/settings.yml")?;
        
        let time_zone: Tz = Tz::from_str(settings.time_zone())?;  // 시간대를 변수로 받음
        let local_time = Utc::now().with_timezone(&time_zone); 
        let new_log_file = format!("vector-{}-{:02}-{:02}.log", local_time.year(), local_time.month(), local_time.day());
        
        // match Tz::from_str(settings.time_zone()) {
        //     Ok(time_zone) => {

        //         let local_time = Utc::now().with_timezone(&time_zone);

        //     }
        //     Err(e) => {
        //         anyhow!("test")
        //     }
        // }
        
        // match Tz::from_str(input_time_zone) {
        //     Ok(time_zone) => {
        //         let local_time = Utc::now().with_timezone(&time_zone);
    
        //         // 년, 월, 일 추출
        //         let year = local_time.year();
        //         let month = local_time.month();
        //         let day = local_time.day();
    
        //         println!("현재 시간대 {}의 년: {}, 월: {}, 일: {}", input_time_zone, year, month, day);
        //     }
        //     Err(_) => {
        //         eprintln!("유효하지 않은 시간대입니다: {}", input_time_zone);
        //     }
        // }


        Ok(())
    }


    

}