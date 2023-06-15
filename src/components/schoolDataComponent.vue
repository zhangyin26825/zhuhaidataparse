<script  lang="ts">
import { defineComponent } from 'vue';
import {SchoolData} from '../dataType.ts';
 
export default defineComponent({  
    name: 'schoolDataComponent',  
    props: {
      schoolData: {
       default: () => ({} as SchoolData)
      //  default: () => ({} as any)
      },  
      //schoolData: SchoolData
    },
 })

</script>

<template>
 <div class="contain">
  <table style="border:1px solid #000; width: 100%" border="1" cellspacing="0">
    <tr>
      <th colspan="4">
        考点数据提交确认书
      </th>
    </tr>
    <tr>
      <td width="15%">考点学校代码</td>
      <td width="35%">{{schoolData.school_code}}</td>
      <td width="15%">考点学校名称</td>
      <td>{{schoolData.school_name}}</td>
    </tr>
    <tr>
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
    </tr>
    <tr>
      <td colspan="4" style="height: 150px">不通过数量及原因：asdsdfsfsfsfsfdsf我是对方是否教师端还副驾是东航飞机收到回复就是房间是否手机东航飞机收到回复是甲方时间的话副书记看粉红色的接口返回社交悍匪四大皆空会发生金凤凰手机号的副驾是可恢复胜多负少胜多负少看风景送点肯定粉红色的接口还范围和人文金凤凰的接口混分巨兽打好饭教师端还风刀霜剑水电费爽肤水打开进球诶去哦UI哦好发黑发asdsdfsfsfsfsfdsf我是对方是否教师端还副驾是东航飞机收到回复就是房间是否手机东航飞机收到回复是甲方时间的话副书记看粉红色的接口返回社交悍匪四大皆空会发生金凤凰手机号的副驾是可恢复胜多负少胜多负少看风景送点肯定粉红色的接口还范围和人文金凤凰的接口混分巨兽打好饭教师端还风刀霜剑水电费爽肤水打开进球诶去哦UI哦好发黑发asdsdfsfsfsfsfdsf我是对方是否教师端还副驾是东航飞机收到回复就是房间是否手机东航飞机收到回复是甲方时间的话副书记看粉红色的接口返回社交悍匪四大皆空会发生金凤凰手机号的副驾是可恢复胜多负少胜多负少看风景送点肯定粉红色的接口还范围和人文金凤凰的接口混分巨兽打好饭教师端还风刀霜剑水电费爽肤水打开进球诶去哦UI哦好发黑发asdsdfsfsfsfsfdsf我是对方是否教师端还副驾是东航飞机收到回复就是房间是否手机东航飞机收到回复是甲方时间的话副书记看粉红色的接口返回社交悍匪四大皆空会发生金凤凰手机号的副驾是可恢复胜多负少胜多负少看风景送点肯定粉红色的接口还范围和人文金凤凰的接口混分巨兽打好饭教师端还风刀霜剑水电费爽肤水打开进球诶去哦UI哦好发黑发</td>
    </tr>
    <tr>
      <td>提交人签名：</td>
      <td>xxx</td>
      <td>接收人签名：</td>
      <td>xxx</td>
    </tr>
  </table>
</div>
</template>

<style scoped>
    @page {
      size: A4 portrait; /*  */
      margin: 3.7cm 2.6cm 3.5cm; /* 国家标准公文页边距 GB/T 9704-2012 */
    }
    @page { margin-top: 0; }

    @page { margin-bottom: 0; }

    @page { margin: 0; }
    body{
      margin: 25mm 25mm 25mm 25mm;
    }
    .contain{
      width: 100%;
      height:842px;
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
</style>
