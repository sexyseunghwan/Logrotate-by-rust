use crate::common::*;


pub trait FileService {
    fn get_file_by_pattern_matching(&self, dir: &str, file_name_pattern: &str) -> Result<Vec<String>, anyhow::Error>;
    fn read_yml_from_file<T: DeserializeOwned>(&self, file_path: &str) -> Result<T, anyhow::Error>;
    fn get_filter_file_not_match(&self, log_files: &Vec<String>, pattern: &str) -> Result<Vec<String>, anyhow::Error>;
    fn get_filter_file_match(&self, log_files: &Vec<String>, pattern: &str) -> Result<Vec<String>, anyhow::Error>;
    fn get_last_compress_log_file(&self, log_files: &Vec<String>) -> Result<String, anyhow::Error>;
}

#[derive(Serialize, Deserialize, Debug, new)]
pub struct FileServicePub;

impl FileService for FileServicePub {
    
    #[doc = "특정 패턴에 맞는 파일이름들을 모두 가져와주는 함수"]
    fn get_file_by_pattern_matching(&self, dir: &str, file_name_pattern: &str) -> Result<Vec<String>, anyhow::Error> {

        let pattern = format!("{}/**/{}", dir, file_name_pattern);
        let mut log_files = Vec::new();

        // glob 패턴 매칭 시 발생할 수 있는 에러 처리
        let entries = glob(&pattern)?;
        
        for entry in entries {
            match entry {
                Ok(path) => {
                    log_files.push(path.display().to_string());
                }
                Err(e) => {
                    error!("[Error][get_file_by_pattern_matching()] Error occurred while reading the file.: {:?}", e);
                }
            }
        }      
        
        Ok(log_files)
    }
    

    #[doc = "패턴 매칭에 대응되지 않는 파일을 반환해주는 함수"]    
    fn get_filter_file_not_match(&self, log_files: &Vec<String>, pattern: &str) -> Result<Vec<String>, anyhow::Error> {
        
        let mut filtered_by_regex: Vec<String> = Vec::new();

        //let pattern = r"_\d{4}_\d{2}_\d{2}_\d+";
        let re = Regex::new(pattern)?;

        for log_file in log_files {
            if ! re.is_match(log_file) {
                filtered_by_regex.push(log_file.to_string());
            }
        }

        Ok(filtered_by_regex)
    }

    
    #[doc = "패턴 매칭에 대응되는 파일을 반환해주는 함수"]
    fn get_filter_file_match(&self, log_files: &Vec<String>, pattern: &str) -> Result<Vec<String>, anyhow::Error> {

        let mut filtered_by_regex: Vec<String> = Vec::new();

        let re = Regex::new(pattern)?;

        for log_file in log_files {
            if re.is_match(log_file) {
                filtered_by_regex.push(log_file.to_string());
            }
        }

        Ok(filtered_by_regex)
    }   
    
    
    #[doc = "패턴 매칭에 대응되는 파일을 반환해주는 함수"]
    fn get_last_compress_log_file(&self, log_files: &Vec<String>) -> Result<String, anyhow::Error> {

        let re = Regex::new(&format!(r"{}(\d+).*", date_pattern))?; 



    }

    
    #[doc = "yml 파일을 읽어서 인스턴스화 해주는 함수"]
    fn read_yml_from_file<T: DeserializeOwned>(&self, file_path: &str) -> Result<T, anyhow::Error> {
    
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let data = serde_yml::from_reader(reader)?;
    
        Ok(data)
    }


    
    


}

