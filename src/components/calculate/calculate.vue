<script setup lang="ts">
import type { FormInst, StepsProps } from 'naive-ui'
import { MdArrowRoundBack, MdArrowRoundForward } from '@vicons/ionicons4'

const loading = ref<boolean>(false)
const current = ref<number | null>(1)
const currentStatus = ref<StepsProps['status']>('process')

const formRef = ref<FormInst | null>(null)

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

const dynamicForm = reactive({
  name: '',
  hobbies: [{ hobby: '' }]
})

const removeItem = (index: number) => {
  dynamicForm.hobbies.splice(index, 1)
}

const addItem = () => {
  dynamicForm.hobbies.push({ hobby: '' })
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
