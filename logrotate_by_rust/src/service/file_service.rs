use crate::common::*;


pub trait FileService {
    fn get_file_by_pattern_matching(&self) -> Result<Vec<String>, anyhow::Error>;
}

pub struct FileServicePub;


impl FileService for FileServicePub {
    
    #[doc = "docs"]
    fn get_file_by_pattern_matching(&self) -> Result<Vec<String>, anyhow::Error> {
            
        let test: Vec<String> = Vec::new();
        


        Ok(test)
    }

}

