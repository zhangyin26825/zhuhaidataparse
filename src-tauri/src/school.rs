use lazy_static::lazy_static;
use serde::Deserialize; 
use std::collections::HashMap;
use std::fs::File;
use std::env; 
// include![concat!(env!("CARGO_MANIFEST_DIR"), "/school.csv")];

#[derive(Debug, Deserialize)]  
pub struct Record {  
    pub school_code: String, 
    pub school_name: String,  
    pub wuli_count: i32,
    pub huaxue_count:i32,
}  

lazy_static! {  
    pub static ref SCHOOL_INFO_MAP: HashMap<String, Record> = {
        let  current_dir = env::current_dir().unwrap();  
        let file = File::open(&current_dir.join("school.csv")).unwrap();  
        let mut rdr=csv::Reader::from_reader(file);
        let mut map = HashMap::new();
        for record in rdr.deserialize() {  
            let record: Record = record.unwrap();  
            map.insert(record.school_code.clone(), record);
        }  
        map    
    };
}

pub fn init_csv(){
    println!("解析csv获取到的学校数量为： {}",SCHOOL_INFO_MAP.len());
}

mod tests {

    #[test]
    fn test_init_school() {
        // assert_eq!(parse_dir("D:\\rust"), 0);
        init_csv();
    }
    
    
}