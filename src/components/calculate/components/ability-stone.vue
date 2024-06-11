<script setup lang="ts">
import type { FormInst, FormRules } from 'naive-ui'
import { buffOptions, debuffOptions } from "../config"

const formRef = ref<FormInst | null>(null)

const buffOptionRef = ref(JSON.parse(JSON.stringify(buffOptions)))

const stoneFormValue = ref<{
  buff_1: {
    code: number | string,
    value: number
  },
  buff_2: {
    code: number | string,
    value: number
  },
  debuff: {
    code: number | string,
    value: number
  }
}>({
  buff_1: {
    code: "",
    value: 0
  },
  buff_2: {
    code: "",
    value: 0
  },
  debuff: {
    code: "",
    value: 0
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
  },
  debuff: {
    code: {
      required: true,
      message: '请选择负面刻印',
      trigger: ['change']
    },
    value: {
      required: true,
      message: '请输入刻印数',
      type: 'number',
      trigger: ['input', 'blur']
    }
  },
}

const handleValidateClick = () => {
  formRef.value?.validate((errors) => {
    if (!errors) {
      console.log('验证通过')
    } else {
      console.log(errors)
    }
  })
}

const buffValueChange = () => {
  // 需要根据选择的刻印值, 给与 disabled, 并放开其他刻印, 便于另一个刻印选择
  buffOptionRef.value = buffOptions.map((item) => {
    if (item.value === stoneFormValue.value.buff_1.code || item.value === stoneFormValue.value.buff_2.code) {
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
</script>

<template>
  <div>
    <n-form ref="formRef" inline :label-width="80" :model="stoneFormValue" :rules="rules">
      <n-form-item label="能力石刻印1" path="buff_1.value">
        <n-select v-model:value="stoneFormValue.buff_1.code" filterable placeholder="选择刻印" :options="buffOptionRef"
          @blur="buffValueChange" />
        <n-input-number class="pl-1" v-model:value="stoneFormValue.buff_1.value" :max="10" :min="0" />
      </n-form-item>
      <n-form-item label="能力石刻印2" path="buff_2.value">
        <n-select v-model:value="stoneFormValue.buff_2.code" filterable placeholder="选择刻印" :options="buffOptionRef"
          @blur="buffValueChange" />
        <n-input-number class="pl-1" v-model:value="stoneFormValue.buff_2.value" :max="10" :min="0" />
      </n-form-item>
      <n-form-item label="能力石负面刻印" path="debuff.value">
        <n-select v-model:value="stoneFormValue.debuff.code" filterable placeholder="选择刻印" :options="debuffOptions" />
        <n-input-number class="pl-1" v-model:value="stoneFormValue.debuff.value" :max="10" :min="0" />
      </n-form-item>
      <n-form-item>
        <n-button attr-type="button" @click="handleValidateClick">
          确认设定
        </n-button>
      </n-form-item>
    </n-form>

    <pre>{{ JSON.stringify(stoneFormValue, null, 2) }}</pre>
  </div>
</template>

<style scoped></style>
