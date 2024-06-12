<script setup lang="ts">
import type { StepsProps } from 'naive-ui'
import ShowAccessory from './components/show-accessory.vue';

const current = ref<number | null>(1)
const currentStatus = ref<StepsProps['status']>('process')
const showAccessoryRef = ref<InstanceType<typeof ShowAccessory> | null>(null)

const calculatePageParam = reactive<Calculate.CalculatePageParam>({
  need_builds: [] as Calculate.Build[],
  stone_builds: {} as Calculate.StoneBuild,
  self_builds: {} as Calculate.SelfBuild,
})

const calculateResultParam = reactive<[Calculate.CalculatePriceParam[], Calculate.CalculateResult["result_array"]]>([
  [], []
])

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

  showAccessoryRef.value?.calculate()
}
</script>

<template>
  <n-space vertical>
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
        <SettingPrice v-show="current === 4" :calculate-page-param="calculatePageParam" @setting-success="priceSettingSuccess" @go-to-prev="prev" />
      </n-step>
      <n-step title="结算">
        <ShowAccessory v-show="current === 5" ref="showAccessoryRef" :calculate-price-param="calculateResultParam" @go-to-prev="prev" />
      </n-step>
    </n-steps>
  </n-space>
</template>

<style lang="scss"></style>
