use crate::common::*;

use crate::service::comprs_service::*;
use crate::service::file_service::*;

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
    pub fn log_rotate_main_handler(&self) -> Result<(), anyhow::Error> {
        
        let settings: SettingsList = self.file_service.read_yml_from_file("./config/settings.yml")?;
        let filtered_pattern = r".*_\d{4}_\d{2}_\d{2}_\d+";

        // 로깅 대상이 되는 디렉토리를 순차적으로 처리.
        for setting in settings {

            let time_zone: Tz = Tz::from_str(setting.time_zone())?;  // 시간대를 변수로 받음
            let local_time = Utc::now().with_timezone(&time_zone); 
            let date_pattern = format!("_{}_{:02}_{:02}_", local_time.year(), local_time.month(), local_time.day());
            
            println!("{:?}", date_pattern);
            //let new_log_format = format!("vector-{}-{:02}-{:02}.log", local_time.year(), local_time.month(), local_time.day());
            
            let file_list = self.file_service.get_file_by_pattern_matching(setting.log_path(), setting.file_name_pattern())?;
            let not_match_list = self.file_service.get_filter_file_not_match(&file_list, filtered_pattern)?;
            let match_list = self.file_service.get_filter_file_match(&file_list, &date_pattern)?;
            
            println!("{:?}", match_list);
            
            let re = Regex::new(&format!(r"{}(\d+).*", date_pattern))?;
            
            let test = match_list
                .iter()
                .filter_map(|file| {
                    if let Some(captures) = re.captures(file) {
                        // 캡처된 숫자를 usize로 변환
                        let num = captures[1].parse::<usize>().ok()?;
                        Some((file, num))
                    } else {
                        None
                    }
                })
                .max_by(|(_, num1), (_, num2)| num1.cmp(num2)) // 숫자를 비교하여 가장 큰 숫자를 찾음
                .map(|(file, _)| file.clone()).unwrap(); // 해당 파일 경로를 반환

            println!("{:?}", test);
                
        }
        
        
        // for elem in settings {
        //     println!("{:?}", elem);
        // }
        
        // 멀티스레드 구성.
        // let time_zone: Tz = Tz::from_str(settings.time_zone())?;  // 시간대를 변수로 받음
        // let local_time = Utc::now().with_timezone(&time_zone); 
        // let new_log_file = format!("vector-{}-{:02}-{:02}.log", local_time.year(), local_time.month(), local_time.day());
        
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