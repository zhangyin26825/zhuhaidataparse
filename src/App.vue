<script lang="ts">
import { Tabs,message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { SchoolData } from './dataType';
import schoolDataComponent from './components/schoolDataComponent.vue';
export default {
  data() {
    return {
      activeKey:'',
      schoolDatas:[] as SchoolData[]
    }
  },
  components: {  
    Tabs,  schoolDataComponent,message
  },  
  methods: {
    async onCLick() {
      const selected = await open({
          directory: true,
          multiple: false,
      });
      console.log(selected)
      invoke('parse_dir', { pathStr: selected })
      .then((response)=>{
        if ((response as []).length === 0) { 
          message.error('当前选择的硬盘找不到答卷信息');
        }else{
          console.log(response);
          this.schoolDatas = response as SchoolData[];
          this.activeKey = this.schoolDatas[0].school_code;
        }
      })
    },
    printHtml(){
      window.print()
    },
    clearData(){
      this.schoolDatas = [];
      invoke('clear_data', {})
      .then((response)=>{
          console.log(" 接收 rust clear_data 返回 "+response)
      })
    }
  }
}
</script>
<template>
   <div class="header_container">
          <a-button type="primary" size="large" @click="onCLick" >选择硬盘</a-button>
          <a-button type="primary" size="large" @click="clearData">清空数据</a-button>
          <a-button type="primary" size="large" >上报数据</a-button>
          <a-button type="primary" size="large" @click="printHtml">打印</a-button>
  </div>
  <div>  
    <a-tabs v-model:activeKey="activeKey">  
      <a-tab-pane v-for="schoolData in schoolDatas" :tab="schoolData.school_name" :key="schoolData.school_code">  
         <schoolDataComponent :schoolData="schoolData"></schoolDataComponent>
      </a-tab-pane>  
    </a-tabs>  
  </div>  
</template>
<style scoped>

.header_container{
  display: flex;  
  flex-wrap: wrap;
}
.header_container .ant-btn{
  flex: 1;  
  margin-right: 8px;
  margin-bottom: 12px;
}

</style>


