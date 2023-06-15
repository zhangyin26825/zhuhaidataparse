<script lang="ts">
import { Tabs } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import { SchoolData } from './dataType';
import schoolDataComponent from './components/schoolDataComponent.vue';
export default {
  data() {
    return {
      s:String,
      schoolDatas:[] as SchoolData[]
    }
  },
  components: {  
    Tabs,  schoolDataComponent
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
        this.schoolDatas = response as SchoolData[];
        // console.log(JSON.stringify(response));
      })
    },
    printHtml(){
      window.print()
    },
    clearData(){
      this.schoolDatas = [];
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
    <a-tabs v-model="schoolDatas">  
      <a-tab v-for="(schoolData, index) in schoolDatas" :tab="schoolData.school_name"   :key="index" :label="schoolData.school_name">  
         <schoolDataComponent :schoolData="schoolData"></schoolDataComponent>
      </a-tab>  
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
@media print{
  .header_container{
    display: none;
  }
  .ant-tabs-nav-wrap{
    display: none;
  }

}
</style>


