


use std::borrow::BorrowMut;
use std::fs;
use std::collections::HashSet;
use std::path::Path;
use std::vec::Vec;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use md5::{Md5, Digest};
use std::io::BufReader;
use std::io::Read;
use hex::ToHex; 
use std::fs::read_to_string; 
use std::hash::{Hash, Hasher}; 
pub use super::school::SCHOOL_INFO_MAP;
pub use super::school::Record;


static mut CHECK_MD5: i32 = 0;
#[tauri::command]
pub fn parse_dir(path_str:&str)->Vec<School>{
    println!("{}",path_str);
    let  dir = fs::read_dir(path_str).unwrap();
    let mut result=vec![];
    for entry in dir{
        let school_dir = entry.unwrap();  
        let path = school_dir.path(); 
        if path.is_dir() { 
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if let Some(record) = SCHOOL_INFO_MAP.get(file_name) {  
                println!("找到学校代码: {}", record.school_code);
                let mut school=School::new(&file_name,record);
                parse_school_code_dir(&path,&mut school);
                result.push(school);
            } else {  
                println!("没有找到学校代码");  
            }  
        }
        println!("{}",&path.display());
    }
    result
}
#[tauri::command]
pub fn clear_data(){
    println!(" rust clear data");
    unsafe{
        CHECK_MD5=0;
    }
}


#[derive(Debug,Serialize)]
pub struct School {  
    school_code: String,  
    school_name: String,  
    wuli_count: i32,
    huaxue_count:i32,
    answer_papers: HashSet<StudentPaper>,
    error_dirs: Vec<String>
}
impl School{
    fn new(school_code:&str,record:& Record)->Self{
        let set=HashSet::new();
        let error_dirs=vec![];
        School{
            school_code:school_code.to_string(),
            school_name:record.school_name.clone(),
            wuli_count:record.wuli_count,
            huaxue_count:record.huaxue_count,
            answer_papers:set,
            error_dirs:error_dirs
        }
    }
}
#[derive(Debug,Serialize)]
struct StudentPaper {
    student_id: String,
    subject: String,
    test_paper_num:String,
    status:bool,
    error_messages: Vec<String>
}


impl PartialEq for StudentPaper {  
    fn eq(&self, other: &StudentPaper) -> bool {  
        (self.student_id.as_bytes(), self.subject.as_bytes(),self.test_paper_num.as_bytes()) 
        == (other.student_id.as_bytes(), other.subject.as_bytes(),other.test_paper_num.as_bytes())  
    }  
}  
  
impl Eq for StudentPaper {} 
impl Hash for StudentPaper {  
    fn hash<H>(&self, state: &mut H)  
    where  
        H: Hasher,  
    {  
        (self.student_id.as_bytes(), self.subject.as_bytes(),self.test_paper_num.as_bytes()).hash(state);  
    }  
} 


fn parse_school_code_dir(path:&Path,school: &mut School ){
    println!("path : {:?}  school: {:?}",path,school);
    let entries = fs::read_dir(path).unwrap();  
    for entry in entries {  
        let entry = entry.unwrap();  
        let path = entry.path();  
        if path.is_dir() {  
            let subject = path.file_stem().unwrap().to_str().unwrap();
            if subject.eq_ignore_ascii_case("wuli") || subject.eq_ignore_ascii_case("huaxue") {
                parse_subject_dir(&path,school,&subject.to_lowercase());
            }else{
                school.error_dirs.push(format!("学科异常目录 {}", subject));
            }
        } 
    }  
}

fn parse_subject_dir(path:&Path,school: &mut School,subject:&str){
    let entries = fs::read_dir(path).unwrap();  
    for entry in entries {  
        let entry = entry.unwrap();  
        let path = entry.path();  
        if path.is_dir() {  
            let test_paper_num = path.file_stem().unwrap().to_str().unwrap();
            if test_paper_num == "H_1001" || test_paper_num == "H_1002" || test_paper_num == "H_1003"
               || test_paper_num == "W_2001" || test_paper_num == "W_2002" || test_paper_num == "W_2003"
            {
                parse_test_paper_num(&path,school,subject,&test_paper_num.to_uppercase());
            } else{
                school.error_dirs.push(format!("试卷异常目录 {}", subject));
            }
        } 
    }  
}

fn parse_test_paper_num(path:&Path,school: &mut School,subject:&str,test_paper_num:&str){
    let entries = fs::read_dir(path).unwrap();  
    for entry in entries {  
        let entry = entry.unwrap();  
        let path = entry.path();  
        if path.is_dir() {  
            let student_id = path.file_stem().unwrap().to_str().unwrap();
            parse_student_id_num(&path,school,subject,test_paper_num,student_id);
        } 
    }  
}

fn need_check_md5()->bool{
    unsafe{
        if CHECK_MD5==0{
            CHECK_MD5+=1;
            return true;
        }
        else {
            return false;
        }
    }
}


fn parse_student_id_num(parent_path:&Path,school: &mut School,subject:&str,test_paper_num:&str,student_id:&str){
    let entries = fs::read_dir(parent_path).unwrap(); 
    let mut has_top_mp4 = false;
    let mut has_side_mp4 = false;
    let mut has_top_mp4_md5 = false;
    let mut has_side_mp4_md5 = false;
    let mut has_json = false;
    let check_md5:bool=need_check_md5();
    let mut topmd5=String::new(); 
    let mut sidemd5=String::new(); 
    let mut topmd5file=String::new(); 
    let mut sidemd5file=String::new(); 
    let mut messages:Vec<String>=vec![];
    let base_dir=parent_path.display();
    if !student_id.starts_with(&school.school_code){
        messages.push("学生ID不以学校代码开头".to_string());
    }

    for entry in entries {  
        let entry = entry.unwrap();  
        let path = entry.path();  
        if path.is_file() {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            if file_name.eq_ignore_ascii_case("side.mp4")  {
                has_side_mp4=true;
                if check_md5 {
                    sidemd5=md5_file(&path)
                }
            }
            if file_name.eq_ignore_ascii_case("top.mp4")  {
                has_top_mp4=true;
                if check_md5 {
                    topmd5=md5_file(&path)
                }
            }
            if file_name.eq_ignore_ascii_case("side.md5")  {
                has_side_mp4_md5=true;
                sidemd5file=read_to_string(&path).unwrap();
            }
            if file_name.eq_ignore_ascii_case("top.md5")  {
                has_top_mp4_md5=true;
                topmd5file=read_to_string(&path).unwrap();
            }
            if file_name.eq_ignore_ascii_case("value.json")  {
                has_json=true;
                let answer_json_data=get_answer_json_data(&path);
                match answer_json_data {
                    Ok(answer_json)=>{
                        match answer_json.answer_paper_id {
                            Some(_) => {},
                            None => {
                                messages.push(format!("解析{}下的value.json文件,获取不到answerPaperId",base_dir)) 
                            },
                        }

                    }
                    Err(error)=>{
                        messages.push(format!("解析{}下的value.json文件出现异常:{}",base_dir,error.to_string()))
                    }
                }     
            }
        } 
    }  
    if !has_top_mp4 {
        messages.push(format!("{}目录下没有top.mp4视频文件",base_dir));
    }
    if !has_side_mp4 {
        messages.push(format!("{}目录下没有side.mp4视频文件",base_dir));
    }
    if !has_top_mp4_md5{
        messages.push(format!("{}目录下没有top.md5文件",base_dir));
    }
    if !has_side_mp4_md5{
        messages.push(format!("{}目录下没有side.md5文件",base_dir));
    }
    if !has_json{
        messages.push(format!("{}目录下没有value.json文件",base_dir));
    }
    if has_top_mp4&&has_top_mp4_md5&&check_md5&&topmd5!=topmd5file {
        messages.push(format!("{}目录下top.mp4视频文件的Md5校验不通过",base_dir));
    }
    if has_side_mp4&&has_side_mp4_md5&&check_md5&&sidemd5!=sidemd5file{
        messages.push(format!("{}目录下side.mp4视频文件的Md5校验不通过",base_dir));
    }
    let status=messages.is_empty();
    let student_paper=StudentPaper{
        student_id:student_id.to_string(),
        test_paper_num: test_paper_num.to_string(),
        subject:subject.to_string(),
        status:status,
        error_messages:messages
    };
    school.answer_papers.borrow_mut().insert(student_paper);
}

fn md5_file(path: &Path) -> String {
    let mut hasher = Md5::new();
    let mut file = std::fs::File::open(path).unwrap();
    let mut buf = [0; 4096];
    let mut buf_reader=BufReader::new(&mut file);
    loop {
        let n = buf_reader.read(&mut buf).unwrap();
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    let hash =hasher.finalize().to_vec();
    return hash.encode_hex();
}

// fn mp4_metadata(path:&Path,messags:&Vec<String>)-> Result<()>{
//     let mut f = File::open(path).unwrap(); 
//     let size = f.metadata()?.len();
//     let reader = BufReader::new(f);
//     let mp4 = mp4::Mp4Reader::read_header(reader, size)?;



//     Ok(())
// }


fn get_answer_json_data(path: &Path)->Result<AnswerJsonData, Error>{
    let mut file = std::fs::File::open(path).unwrap();  
    let mut content = String::new();  
    file.read_to_string(&mut content).unwrap();
    serde_json::from_str::<AnswerJsonData>(&content)
}


#[derive(Debug, Deserialize, Serialize)]  
struct AnswerJsonData { 
    #[serde (rename = "answerPaperId")] 
    answer_paper_id: Option<String>, 
    #[serde (rename = "testPaperNumber")]  
    test_paper_number: Option<String>, 
    #[serde (rename = "studentId")]   
    student_id: Option<String>,  
    #[serde (rename = "textAnswer")]   
    text_answer: Vec<TextAnswer>,  
}  
  
#[derive(Debug, Deserialize, Serialize)]  
struct TextAnswer {  
    #[serde (rename = "serialNumber")]   
    serial_number: i32,  
    #[serde (rename = "text")]   
    text: String,  
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_dir() {
        // assert_eq!(parse_dir("D:\\rust"), 0);
    }
    
    
     #[test]
    fn test_md5_file() {
       let path= Path::new("D:\\zhuhaivideo\\test\\17001\\huaxue\\H_1002\\170010001\\top.mp4");
       let s= md5_file(&path); 
        assert_eq!(s, "8cfb848c2106322ea32ff750d4194236");
    }

    #[test]
    fn test_get_answer_json_data() {
       let path= Path::new("D:\\zhuhaivideo\\test\\17001\\wuli\\W_2002\\170010005\\value.json");
       let s= get_answer_json_data(&path); 
       println!("{:?}",s)
    }
    
    
    
}




