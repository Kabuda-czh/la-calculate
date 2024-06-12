<script setup lang="ts">
import type { StepsProps } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";

const loading = ref<boolean>(false)
const current = ref<number | null>(1)
const currentStatus = ref<StepsProps['status']>('process')

const calculatePageParam = reactive<Calculate.CalculatePageParam>({
  need_builds: [] as Calculate.Build[],
  stone_builds: {} as Calculate.StoneBuild,
  self_builds: {} as Calculate.SelfBuild,
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
}
const priceSettingSuccess = (calculatePriceParam: Calculate.CalculatePriceParam[], resultArray: Calculate.CalculateResult["result_array"]) => {
  calculateResultParam[0] = calculatePriceParam
  calculateResultParam[1] = resultArray
  next()

async function calculate() {
  console.log(await invoke("calculate", { buildParam: calculatePageParam }))
}
</script>

<template>
  <n-space vertical>
        <SettingPrice v-show="current === 4" :calculate-page-param="calculatePageParam" @setting-success="priceSettingSuccess" @go-to-prev="prev" />
  </n-space>
</template>

<style lang="scss"></style>
