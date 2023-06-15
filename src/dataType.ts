
export interface AnswerPaper {  
    error_messages: string[];  
    status: boolean;  
    student_id: string;  
    subject: string;  
    test_paper_num: string;  
  }  
    
export interface SchoolData {  
    error_dirs: string[];  
    school_code: string;  
    school_name: string; 
    answer_papers: AnswerPaper[];
}

export interface SchoolDataArray extends Array<SchoolData>{
    items:SchoolData[]
}