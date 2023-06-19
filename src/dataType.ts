
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
    wuli_count: number,
    huaxue_count:number, 
    answer_papers: AnswerPaper[];
}

export interface SubjectStat{
    subject: string;
    all:number;
    correct:number;
    error:number;
    miss:number;
}

export interface SchoolDataArray extends Array<SchoolData>{
    items:SchoolData[]
}