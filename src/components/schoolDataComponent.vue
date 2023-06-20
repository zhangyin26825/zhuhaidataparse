<script  lang="ts">
import { defineComponent,  } from 'vue';
import {SchoolData,SubjectStat} from '../dataType.ts';
 
export default defineComponent({  
    name: 'schoolDataComponent',  
    props: {
      schoolData: {
       default: () => ({} as SchoolData)
      //  default: () => ({} as any)
      }
    },
    data() {
      return {
      }
    },
    computed:{

      subjectStats(){
            let school:SchoolData=this.schoolData;
            let subjects=["huaxue","wuli"];
            let subjectStatsTemp=[];
            for (var index in subjects) { 
              var subject = subjects[index]; 
              let all:number=subject=="huaxue"?school.huaxue_count:school.wuli_count;
              let subjectAnswerPaper=school.answer_papers.filter(i=>i.subject==subject);
              let correctNumber=subjectAnswerPaper.filter(i=>i.status).length;
              let error= subjectAnswerPaper.length-correctNumber;
              if (subjectAnswerPaper.length!=0){
                subjectStatsTemp.push({
                  subject:subject=="huaxue"?"化学":"物理",
                  all:all,
                  correct:correctNumber,
                  error:error,
                  miss:all-correctNumber,
                } as SubjectStat);
              }
            }
            console.log("invoke computed    subjectStats.length: "+subjectStatsTemp.length)
            return subjectStatsTemp;
      },
      errorAnswerPaperMessages(){
        const filteredAnswers = this.schoolData.answer_papers.filter(answer => !answer.status);  
        const errorMessages = filteredAnswers.flatMap(answer => answer.error_messages);
        return errorMessages;
      },
      today(){
        var today = new Date();  
        // 获取年、月、日信息  
        var year = today.getFullYear();  
        var month = today.getMonth() + 1; // 月份从0开始，需要加1  
        var day = today.getDate(); 
        return  year + "年" + month + "月" + day + "日"; 
      }
    },

    watch:{
      
    }
 })

</script>

<template>
 <div class="contain">
  <table style="border:1px solid #000; width: 100%" border="1" cellspacing="0">
    <tr>
      <th colspan="16">
        考点数据提交确认书
      </th>
    </tr>
    <tr>
      <td colspan="3">考点学校代码</td>
      <td colspan="3" >{{schoolData.school_code}}</td>
      <td colspan="3" >考点学校名称</td>
      <td colspan="7" >{{schoolData.school_name}}</td>
    </tr>
    <tr>
      <td colspan="3">应考数量</td>
      <td colspan="3" >{{schoolData.wuli_count}}</td>
      <td colspan="3" >提交日期</td>
      <td colspan="7" >{{today}}</td>
    </tr>
    <tr v-for="subjectStat in subjectStats">
      <td colspan="2"  width="10%">考试科目</td>
      <td colspan="2"  width="10%" >{{subjectStat.subject}}</td>
      <td colspan="2"  width="10%" >正常数量</td>
      <td colspan="2"  width="10%" >{{subjectStat.correct}}</td>
      <td colspan="2"  width="10%" >异常数量</td>
      <td colspan="2"  width="10%" >{{subjectStat.error}}</td>
      <td colspan="2"  width="10%" >缺少数量</td>
      <td colspan="2"  width="10%" >{{subjectStat.miss}}</td>
    </tr>

    <!-- <tr>
      <td>考试科目</td>
      <td>1001</td>
      <td>提交日期</td>
      <td>XX中学</td>
    </tr>
    <tr>
      <td>学科答卷总量</td>
      <td>1001</td>
      <td>检查通过数量</td>
      <td>XX中学</td>
    </tr> -->
    <tr>
      <td colspan="16" style="height: 150px">不通过数量及原因：
        <template v-for="errorMessages in errorAnswerPaperMessages">
            {{ errorMessages }} <br>
        </template>
      </td>
    </tr>
    <tr>
      <td colspan="4" >提交人签名：</td>
      <td colspan="4" ></td>
      <td colspan="4" >接收人签名：</td>
      <td colspan="4" ></td>
    </tr>
  </table>
</div>
</template>

<style scoped>
    /* body{
      margin: 25mm 25mm 25mm 25mm;
    } */
    .contain{
      /* margin: 25mm 20mm 25mm 20mm; */
      width: 100%;
      height:842px;
      page-break-before: always;
      page-break-after:always;
    }
    table,td{
      border:1px solid #000;
      border-collapse: collapse;
      padding-top: 10px;
      padding-bottom: 10px;
      padding-left: 5px;
    }
    td{
      height:80px;
    }
    th{
      padding-top: 10px;
      padding-bottom: 10px;
      font-size: 20px;
      height: 100px;
    }
    @media print{
      @page {
      size: A4 portrait; /*  */
      /* 国家标准公文页边距 GB/T 9704-2012 */
      /* margin: 3.7cm 2.6cm 3.5cm;  */
      margin-top: 0;
      margin-bottom: 0;
      margin: 0; 
    }
    body{
      /* margin: 25mm 25mm 25mm 25mm; */
    }
    .contain{
      margin: 25mm 10mm 25mm 10mm;
      width: 90%;
      height:29cm;
      page-break-before: always;
      page-break-after:always;
    }
    table,td{
      border:1px solid #000;
      border-collapse: collapse;
      padding-top: 10px;
      padding-bottom: 10px;
      padding-left: 5px;
    }
    td{
      height:80px;
    }
    th{
      padding-top: 10px;
      padding-bottom: 10px;
      font-size: 20px;
      height: 100px;
    }

   }
</style>
