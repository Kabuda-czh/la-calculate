<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui'
import { classesWithBuffOptions } from "../config"

const emit = defineEmits<{
  selfBuildSuccess: [selfBuild: Calculate.SelfBuild]
  goToPrev: []
}>()

const formRef = ref<FormInst | null>(null)

const classesWithBuffOptionRef = ref(JSON.parse(JSON.stringify(classesWithBuffOptions)))

const buffQualityOptions = [
  { label: '高级 - 3', value: 3 },
  { label: '稀有 - 6', value: 6 },
  { label: '英雄 - 9', value: 9 },
  { label: '传说 - 12', value: 12 }
]

const selfBuildFormValue = ref<Calculate.SelfBuild>({
  buff_1: {
    code: "",
    value: 12
  },
  buff_2: {
    code: "",
    value: 12
  }
})

const rules: FormRules = {
  buff_1: {
    code: {
      required: true,
      message: '请选择刻印1',
      trigger: ['change']
    },
    value: {
      required: true,
      message: '请输入刻印数',
      type: 'number',
      trigger: ['input', 'blur']
    }
  },
  buff_2: {
    code: {
      required: true,
      message: '请选择刻印1',
      trigger: ['change']
    },
    value: {
      required: true,
      message: '请输入刻印数',
      type: 'number',
      trigger: ['input', 'blur']
    }
  }
}

const handleValidateClick = () => {
  formRef.value?.validate((errors) => {
    if (!errors) {
      const buildDescription = Object.values(selfBuildFormValue.value).map(item => {
        return `${classesWithBuffOptions.find(option => option.value === item.code)?.label}${item.value}`
      }).join(' ')
      window.$message?.success(`自身刻印设定成功: ${buildDescription}`)
      emit('selfBuildSuccess', selfBuildFormValue.value)
    } else {
      window.$message?.warning('请完善刻印设定')
    }
  })
}

const buffValueChange = () => {
  // 需要根据选择的刻印值, 给与 disabled, 并放开其他刻印, 便于另一个刻印选择
  classesWithBuffOptionRef.value = classesWithBuffOptions.map((item) => {
    if (item.value === selfBuildFormValue.value.buff_1.code || item.value === selfBuildFormValue.value.buff_2.code) {
      return {
        ...item,
        disabled: true
      }
    }
    return {
      ...item,
      disabled: false
    }
  })
}

const goToPrev = () => {
  selfBuildFormValue.value = {
    buff_1: {
      code: "",
      value: 12
    },
    buff_2: {
      code: "",
      value: 12
    }
  }
  emit('goToPrev')
}
</script>

<template>
  <div>
    <n-alert class="mb-5" title="提示" type="warning">
      请选择自身携带的刻印等级
    </n-alert>
    <n-form ref="formRef" inline :label-width="80" :model="selfBuildFormValue" :rules="rules">
      <n-form-item label="携带刻印1" path="buff_1.value">
        <n-select v-model:value="selfBuildFormValue.buff_1.code" filterable placeholder="选择刻印" :options="classesWithBuffOptionRef"
          @blur="buffValueChange" />
        <n-select class="pl-1" v-model:value="selfBuildFormValue.buff_1.value" filterable placeholder="选择刻印品质"
          :options="buffQualityOptions" />
      </n-form-item>
      <n-form-item label="携带刻印2" path="buff_2.value">
        <n-select v-model:value="selfBuildFormValue.buff_2.code" filterable placeholder="选择刻印" :options="classesWithBuffOptionRef"
          @blur="buffValueChange" />
        <n-select class="pl-1" v-model:value="selfBuildFormValue.buff_2.value" filterable placeholder="选择刻印品质"
          :options="buffQualityOptions" />
      </n-form-item>
    </n-form>

    <div>
      <n-button attr-type="button" @click="goToPrev">
        返回上一级
      </n-button>
      <n-button class="ml-1" attr-type="button" @click="handleValidateClick">
        确认设定
      </n-button>
    </div>
  </div>
</template>

<style scoped></style>
