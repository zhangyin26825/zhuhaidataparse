
extern crate md5;
use std::borrow::BorrowMut;
use std::fs;
use std::fs::File;
use lazy_static::lazy_static; 
use std::collections::HashMap;
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


lazy_static! {  
    static ref SCHOOL_CODE_MAP: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("17002","珠海市体育运动学校");
        map.insert("17004","珠海一附实验中学");
        map.insert("17005","北京师范大学珠海分校附属外国语学校");
        map.insert("37010","斗门区白藤湖初级中学");
        map.insert("37011","斗门区斗门镇初级中学");
        map.insert("37012","斗门区城南学校");
        map.insert("37013","斗门区城东中学");
        map.insert("17001","珠海市第四中学");
        map.insert("37014","斗门区少年业余体校");
        map.insert("27007","珠海市湾仔中学");
        map.insert("27008","珠海市拱北中学");
        map.insert("27005","珠海市前山中学");
        map.insert("27006","珠海市南屏中学");
        map.insert("27009","珠海市夏湾中学");
        map.insert("27000","香洲区社青类考生");
        map.insert("27003","珠海市文园中学");
        map.insert("27004","珠海市九洲中学");
        map.insert("27001","珠海市紫荆中学");
        map.insert("27002","珠海市紫荆中学桃园校区");
        map.insert("37015","珠海中特文武学校");
        map.insert("37016","斗门区井岸西埔新徽实验学校");
        map.insert("37017","斗门区湖海学校");
        map.insert("47010","珠海市广东实验中学金湾学校附属初中");
        map.insert("37018","斗门区德恒实验学校");
        map.insert("47011","珠海市金湾区金山实验学校");
        map.insert("37019","斗门区箐华学校");
        map.insert("47009","珠海市南水中学");
        map.insert("47007","珠海市平沙第二中学");
        map.insert("47008","珠海市平沙第三中学");
        map.insert("37000","斗门区社青类考生");
        map.insert("47005","珠海市金湾区景山实验学校");
        map.insert("37001","斗门区实验中学");
        map.insert("47006","珠海市平沙第一中学");
        map.insert("37002","斗门区第二中学");
        map.insert("47003","珠海市三灶中学");
        map.insert("37003","斗门区第四中学");
        map.insert("47004","珠海市小林中学");
        map.insert("27018","珠海东方外语实验学校");
        map.insert("27019","珠海市恩溢学校");
        map.insert("27017","珠海市香洲区明珠中英文学校");
        map.insert("27010","珠海市第五中学");
        map.insert("27011","珠海市第七中学");
        map.insert("27014","珠海市第十中学");
        map.insert("27015","珠海市第十一中学");
        map.insert("27012","珠海市第八中学");
        map.insert("27013","珠海市第九中学");
        map.insert("37004","斗门区斗门镇赤坎初级中学");
        map.insert("47001","珠海市红旗中学");
        map.insert("37005","斗门区乾务镇五山初级中学");
        map.insert("47002","珠海市金海岸中学");
        map.insert("37006","斗门区乾务镇初级中学");
        map.insert("37007","斗门区白蕉镇六乡初级中学");
        map.insert("47000","金湾区社青类考生");
        map.insert("37008","斗门区莲洲镇莲溪学校");
        map.insert("37009","斗门区莲洲镇横山初级中学");
        map.insert("27027","珠海市凤凰中学");
        map.insert("27028","珠海市第十六中学");
        map.insert("27021","珠海市新世纪学校");
        map.insert("29002","珠海市横琴新区哈罗礼德学校");
        map.insert("27022","珠海市香洲区立才学校");
        map.insert("29001","珠海市横琴新区第一中学");
        map.insert("30001","珠海市希望之星实验学校");
        map.insert("29000","横琴新区社青类考生");
        map.insert("27020","珠海市容闳学校");
        map.insert("27025","珠海市第十三中学");
        map.insert("27026","珠海市梅华中学");
        map.insert("27023","珠海市香洲区壮华学校");
        map.insert("27024","珠海市香洲区壮志学校");
        map.insert("29003","珠海市横琴新区华发容闳学校");
        map.insert("37020","珠海市珠峰实验学校");
        map.insert("37021","珠海市斗门区博雅中学");
        map.insert("37022","珠海市斗门区西湖学校");
        map.insert("28000","高新区社青类考生");
        map.insert("28001","珠海中山大学附属中学");
        map.insert("28004","珠海市金峰学校");
        map.insert("28002","珠海市金鼎中学");
        map.insert("28003","珠海高新区青鸟北附实验学校");
        map    
    };
}

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
            if let Some(value) = SCHOOL_CODE_MAP.get(&file_name) {  
                println!("找到学校代码: {}", value);
                let mut school=School::new(&file_name,&value);
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

#[derive(Debug,Serialize)]
pub struct School {  
    school_code: String,  
    school_name: String,  
    answer_papers: HashSet<StudentPaper>,
    error_dirs: Vec<String>
}
impl School{
    fn new(school_code:&str,school_name:& str)->Self{
        let set=HashSet::new();
        let error_dirs=vec![];
        School{
            school_code:school_code.to_string(),
            school_name:school_name.to_string(),
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

fn parse_student_id_num(parent_path:&Path,school: &mut School,subject:&str,test_paper_num:&str,student_id:&str){
    let entries = fs::read_dir(parent_path).unwrap(); 
    let mut hasTopMp4 = false;
    let mut hasSideMp4 = false;
    let mut hasTopMp4Md5 = false;
    let mut hasSideMp4Md5 = false;
    let mut hasJson = false;
    let checkMd5:bool=true;
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
                hasSideMp4=true;
                if checkMd5 {
                    sidemd5=md5_file(&path)
                }
            }
            if file_name.eq_ignore_ascii_case("top.mp4")  {
                hasTopMp4=true;
                if checkMd5 {
                    topmd5=md5_file(&path)
                }
            }
            if file_name.eq_ignore_ascii_case("side.md5")  {
                hasSideMp4Md5=true;
                sidemd5file=read_to_string(&path).unwrap();
            }
            if file_name.eq_ignore_ascii_case("top.md5")  {
                hasTopMp4Md5=true;
                topmd5file=read_to_string(&path).unwrap();
            }
            if file_name.eq_ignore_ascii_case("value.json")  {
                hasJson=true;
                let answerJsonData=get_answer_json_data(&path);
                match answerJsonData {
                    Ok(answerJson)=>{
                        match answerJson.answer_paper_id {
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
    if !hasTopMp4 {
        messages.push(format!("{}目录下没有top.mp4视频文件",base_dir));
    }
    if !hasSideMp4 {
        messages.push(format!("{}目录下没有side.mp4视频文件",base_dir));
    }
    if !hasTopMp4Md5{
        messages.push(format!("{}目录下没有top.md5文件",base_dir));
    }
    if !hasSideMp4Md5{
        messages.push(format!("{}目录下没有side.md5文件",base_dir));
    }
    if !hasJson{
        messages.push(format!("{}目录下没有value.json文件",base_dir));
    }
    if hasTopMp4&&hasTopMp4Md5&&checkMd5&&topmd5!=topmd5file {
        messages.push(format!("{}目录下top.mp4视频文件的Md5校验不通过",base_dir));
    }
    if hasSideMp4&&hasSideMp4Md5&&checkMd5&&sidemd5!=sidemd5file{
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




