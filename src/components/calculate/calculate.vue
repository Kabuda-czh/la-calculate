<script setup lang="ts">
import type { FormInst, StepsProps } from 'naive-ui'
import { MdArrowRoundBack, MdArrowRoundForward } from '@vicons/ionicons4'

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
    <n-steps vertical :current="(current as number)" :status="currentStatus">
      <n-step title="选择想要搭配的刻印">
        <ChooseBuild v-show="current === 1" />
      </n-step>
      <n-step title="选择能力石头">
        <AbilityStone v-show="current === 2" />
      </n-step>
      <n-step title="Break" />
      <n-step title="Come Together" description="Here come old flat top He come grooving up slowly" />
      <n-step title="Something" description="Something in the way she moves Attracts me like no other lover" />
    </n-steps>
    <n-space>
      <n-button-group>
        <n-button @click="prev">
          <template #icon>
            <n-icon>
              <md-arrow-round-back />
            </n-icon>
          </template>
        </n-button>
        <n-button @click="next">
          <template #icon>
            <n-icon>
              <md-arrow-round-forward />
            </n-icon>
          </template>
        </n-button>
      </n-button-group>
      <n-radio-group v-model:value="currentStatus" size="medium" name="vertical">
        <n-radio-button value="error">
          Error
        </n-radio-button>
        <n-radio-button value="process">
          Process
        </n-radio-button>
        <n-radio-button value="wait">
          Wait
        </n-radio-button>
        <n-radio-button value="finish">
          Finish
        </n-radio-button>
      </n-radio-group>
    </n-space>
  </n-space>
</template>

<style lang="scss"></style>
