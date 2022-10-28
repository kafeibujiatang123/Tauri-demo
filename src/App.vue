<template>
  <div id="app">
    <img alt="Vue logo" src="./assets/logo.png">
    <HelloWorld :msg="fullMsg"/>
    <input type="text" v-model="msg">
    <button @click="showMsg">显示</button>
    <button @click="writeFile">写文件</button>
    <div style="margin-top: 10px;">
      <button @click="openWin">打开文件</button>
      <p>{{text}}</p>
    </div>
    <div style="margin-top: 10px;">
      <button @click="showMessage">显示系统dialog</button>
    </div>
    <div style="margin-top: 10px;">
      <button @click="exitFn">退出程序</button>
      <button @click="relaunchFn">重启程序</button>
    </div>
    <div style="background-color: beige;width:100wh;height:30px;">{{version}}</div>
  </div>
</template>

<script>
import HelloWorld from './components/HelloWorld.vue'
import {version} from '../package.json'
import {invoke} from '@tauri-apps/api/tauri'
import {open,message} from '@tauri-apps/api/dialog'
import {writeFile,readTextFile,BaseDirectory} from '@tauri-apps/api/fs'
import {sendNotification } from '@tauri-apps/api/notification'
import os from '@tauri-apps/api';
import { exit,relaunch  } from '@tauri-apps/api/process';


export default {
  name: 'App',
  components: {
    HelloWorld
  },
  data() {
    return {
      version,
      msg:'',
      fullMsg:'',
      text:'',
      path:''
    }
  },
  async mounted() {
    const archName = await os.arch();
    console.log('archName',archName);
    const platformName = await os.platform();
    console.log('platformName',platformName);
    const tempdirPath = await os.tempdir();
    console.log('tempdirPath',tempdirPath);
    const osType = await os.type();
    console.log('osType',osType);
    const osVersion = await os.version();
    console.log('osVersion ',osVersion );
  },
  methods: {
    async showMsg(){
      this.fullMsg = await invoke("greet",{name:this.msg})
    },
    async openWin(){
      // open与系统交互,在本地找文件
      this.path = await open({
        filters:[{
          name:'txt',
          extensions:['txt']
        }]
      })
      // readTextFile读文件中内容
      this.text = await readTextFile(this.path, { dir: BaseDirectory.App });
    },
    async writeFile(){
      // 写入数据到文件中
      await writeFile(this.path,this.msg,{ dir: BaseDirectory.App })
      // 在读一遍在页面展示
      this.text = await readTextFile(this.path, { dir: BaseDirectory.App });
      // 系统通知
      sendNotification({title:'提示',body:'文件写入成功!!!',icon:''})
    },
    showMessage(){
      message("弹窗显示内容")
    },
    async relaunchFn(){
      await relaunch();
    },
    async exitFn(){
      await exit(1);
    }
  },
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
