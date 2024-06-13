<script setup lang="ts">
import type { StepsProps } from 'naive-ui'
import ChooseBuild from './components/choose-build.vue';
import AbilityStone from './components/ability-stone.vue';
import ChooseSelfBuild from './components/choose-self-build.vue';
import SettingPrice from './components/setting-price.vue';
import ShowAccessory from './components/show-accessory.vue';

const current = ref<number | null>(1)
const currentStatus = ref<StepsProps['status']>('process')
const chooseBuildRef = ref<InstanceType<typeof ChooseBuild> | null>(null)
const abilityStoneRef = ref<InstanceType<typeof AbilityStone> | null>(null)
const chooseSelfBuildRef = ref<InstanceType<typeof ChooseSelfBuild> | null>(null)
const settingPriceRef = ref<InstanceType<typeof SettingPrice> | null>(null)
const showAccessoryRef = ref<InstanceType<typeof ShowAccessory> | null>(null)

const showImportModal = ref<boolean>(false)
const showExportModal = ref<boolean>(false)
const jsonTextarea = ref<string>('')

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

const importJsonData = () => {
  const importJsonValue: Calculate.CalculateJson = JSON.parse(jsonTextarea.value.trim())

  calculatePageParam.need_builds = importJsonValue.calculatePageParam.need_builds
  calculatePageParam.stone_builds = importJsonValue.calculatePageParam.stone_builds
  calculatePageParam.self_builds = importJsonValue.calculatePageParam.self_builds

  if (chooseBuildRef && chooseBuildRef.value)
    chooseBuildRef.value.setDynamicFormBuildsValue(importJsonValue.calculatePageParam.need_builds)

  if (abilityStoneRef && abilityStoneRef.value)
    abilityStoneRef.value.setStoneFormValue(importJsonValue.calculatePageParam.stone_builds)

  if (chooseSelfBuildRef && chooseSelfBuildRef.value)
    chooseSelfBuildRef.value.setSelfBuildFormValue(importJsonValue.calculatePageParam.self_builds)

  if (settingPriceRef && settingPriceRef.value) {
    settingPriceRef.value.data = importJsonValue.data
    settingPriceRef.value.artifact_check = importJsonValue.artifactCheck
    settingPriceRef.value.firstCalculate = false
  }

  current.value = 4

  window.$message?.success('导入成功')
  showImportModal.value = false
}

const exportJsonData = () => {
  const jsonData: Calculate.CalculateJson = {
    calculatePageParam,
    artifactCheck: settingPriceRef?.value?.artifact_check as boolean,
    data: settingPriceRef?.value?.data as Calculate.CalculatePriceParam[]
  }

  navigator.clipboard.writeText(JSON.stringify(jsonData))
    .then(() => {
      window.$message?.success('复制成功')
      showExportModal.value = false
    })
}
</script>

<template>
  <n-space vertical>
    <n-steps vertical :current="(current as number)" :status="currentStatus">
      <n-step title="选择想要搭配的刻印">
        <ChooseBuild v-show="current === 1" ref="chooseBuildRef" @build-success="buildSetting" />
      </n-step>
      <n-step title="选择能力石">
        <AbilityStone v-show="current === 2" ref="abilityStoneRef" @stone-build-success="stoneSetting"
          @go-to-prev="prev" />
      </n-step>
      <n-step title="选择自身刻印">
        <ChooseSelfBuild v-show="current === 3" ref="chooseSelfBuildRef" @self-build-success="selfBuildSetting"
          @go-to-prev="prev" />
      </n-step>
      <n-step title="配置首饰金额">
        <SettingPrice v-show="current === 4" ref="settingPriceRef" :calculate-page-param="calculatePageParam"
          @setting-success="priceSettingSuccess" @go-to-prev="prev" />
      </n-step>
      <n-step title="结算">
        <ShowAccessory v-show="current === 5" ref="showAccessoryRef" :calculate-price-param="calculateResultParam"
          @go-to-prev="prev" />
      </n-step>
    </n-steps>
    <n-divider />
    <div class="flex-c gap-5">
      <n-button @click="showImportModal = true">
        导入 json 数据
      </n-button>
      <n-button @click="showExportModal = true">
        导出 json 数据
      </n-button>
    </div>

    <n-modal v-model:show="showImportModal" preset="card">
      <template #header>
        <div>导入 Json 数据</div>
      </template>
      <div>
        <n-input v-model:value="jsonTextarea" type="textarea" size="small" :autosize="{
          minRows: 3,
          maxRows: 5
        }" />
      </div>
      <template #action>
        <div>
          <n-button @click="importJsonData">确定</n-button>
        </div>
      </template>
    </n-modal>

    <n-modal v-model:show="showExportModal" preset="card">
      <template #header>
        <div>Json 数据</div>
      </template>
      <div>
        <n-infinite-scroll style="height: 240px" :distance="10">
          <n-code :code="JSON.stringify({ calculatePageParam, data: settingPriceRef?.data })" language="json" word-wrap />
        </n-infinite-scroll>
      </div>
      <template #action>
        <div>
          <n-button @click="exportJsonData">一键复制</n-button>
        </div>
      </template>
    </n-modal>
  </n-space>
</template>

<style lang="scss"></style>
