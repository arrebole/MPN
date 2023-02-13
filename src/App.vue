<template>
  <h2 class="title">国际短信手机号分析助手</h2>
  <div class="main">
    <input class="input" name="file" type="file" @input="handleInput" />
    <div> <a download="result.csv" :href="state.url">{{ state.lable }}</a></div>
  </div>
</template>



<script lang="ts" setup>
import { reactive } from 'vue';
// import { invoke } from '@tauri-apps/api';
import rules from './rules.json';

const state = reactive({
  url: '',
  lable: '下载结果'
})

function readFile(file: File): Promise<string> {
  const reader = new FileReader()
  reader.readAsText(file);
  return new Promise((resolve) => {
    reader.onload = () => {
      resolve(reader.result as string)
    }
  })
}

function showDownload(data: string) {
  state.url = window.URL.createObjectURL(
    new Blob([data], { type: "text/plain" }),
  );
}

async function handleInput(e: Event) {
  // 文本内容
  // @ts-ignore
  const data = await readFile(e.target.files[0] as File);

  // 手机号池
  const datasource = new Map<string, number>();

  data.split('\n').forEach((line) => {
    const [mobile, count] = line.split(',');
    if (!mobile || isNaN(parseInt(count))) {
      return;
    }

    for (const key in rules) {
      // @ts-ignore
      const regs: RegExp[] = rules[key].map((v: string) => new RegExp(v));
      if (regs.map(r => r.test(mobile)).includes(true)) {
        if (datasource.has(key)) {
          datasource.set(key, datasource.get(key)! + parseInt(count))
        } else {
          datasource.set(key, parseInt(count));
        }
        return
      }
    }
  });

  return showDownload(
    Array.from(datasource.keys()).map(k => `${k}, ${datasource.get(k)}`).join('\n')
  );
}

</script>


<style>
.title {
  text-align: center;
}

.main {
  text-align: center;
  vertical-align: middle;
}

.input {
  padding: 2rem;
  margin: 1.4rem;
}
</style>

