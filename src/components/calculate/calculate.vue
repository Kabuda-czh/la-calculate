<script setup lang="ts">
import type { StepsProps } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";

const loading = ref<boolean>(false)
const current = ref<number | null>(1)
const currentStatus = ref<StepsProps['status']>('process')

const calculatePageParam = reactive<Record<string, any>>({
  need_builds: [] as Calculate.Build[],
  stone_builds: {} as Calculate.StoneBuild,
  self_builds: [] as Calculate.SelfBuild[],
})

const next = () => {
  if (current.value === null) current.value = 1
  else if (current.value >= 5) current.value = null
  else current.value++
}
const prev = () => {
  if (current.value === 0) current.value = null
  else if (current.value === null) current.value = 5
  else current.value--
}

const buildSetting = (needBuilds: Calculate.Build[]) => {
  calculatePageParam.need_builds = needBuilds
  next()
}

const stoneSetting = (stoneBuilds: Calculate.StoneBuild) => {
  calculatePageParam.stone_builds = stoneBuilds
  next()
}

const selfBuildSetting = (selfBuilds: Calculate.SelfBuild) => {
  calculatePageParam.self_builds = selfBuilds
  next()
  calculate()
}

async function calculate() {
  console.log(await invoke("calculate", { buildParam: calculatePageParam }))
}
</script>

<template>
  <n-space vertical>
    <n-spin :show="loading">
      <n-steps vertical :current="(current as number)" :status="currentStatus">
        <n-step title="选择想要搭配的刻印">
          <ChooseBuild v-show="current === 1" @build-success="buildSetting" />
        </n-step>
        <n-step title="选择能力石">
          <AbilityStone v-show="current === 2" @stone-build-success="stoneSetting" @go-to-prev="prev" />
        </n-step>
        <n-step title="选择自身刻印">
          <ChooseSelfBuild v-show="current === 3" @self-build-success="selfBuildSetting" @go-to-prev="prev" />
        </n-step>
        <n-step title="配置首饰金额">
          <div v-show="current === 4">4</div>
        </n-step>
        <n-step title="结算">
          <div v-show="current === 5">5</div>
        </n-step>
      </n-steps>
    </n-spin>
  </n-space>
</template>

<style lang="scss"></style>
