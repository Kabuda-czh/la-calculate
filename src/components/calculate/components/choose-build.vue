<script setup lang="ts">
import type { FormInst } from 'naive-ui'
import { classesWithBuffOptions } from "../config"

const formRef = ref<FormInst | null>(null)

const classesWithBuffOptionsRef = ref(JSON.parse(JSON.stringify(classesWithBuffOptions)))

const dynamicForm = reactive<{
  builds: { code: number | string, level: number }[]
}>({
  builds: [{ code: 44, level: 3 }, { code: "", level: 3 }, { code: "", level: 3 }]
})

const handleValidateClick = () => {
  const sumBuildsLevel = dynamicForm.builds.reduce((prev, current) => prev + current.level * 5, 0)
  window.$message?.success(`总刻印等级为: ${sumBuildsLevel}: 89`)
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
  classesWithBuffOptionsRef.value = classesWithBuffOptions.map((item) => {
    if (dynamicForm.builds.map(build => +build.code).includes(item.value)) {
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

const removeItem = (index: number) => {
  dynamicForm.builds.splice(index, 1)
}

const addItem = () => {
  dynamicForm.builds.push({ code: "", level: 3 })
}
</script>

<template>
  <div>
    <n-form ref="formRef" :model="dynamicForm">
      <n-form-item v-for="(item, index) in dynamicForm.builds" :key="index" :label="`刻印${index + 1}`"
        :path="`builds[${index}].code`" :rule="{
          required: true,
          type: 'number',
          message: `请输入刻印${index + 1}`,
          trigger: ['change']
        }">
        <n-select v-model:value="item.code" filterable placeholder="选择刻印" :options="classesWithBuffOptionsRef"
          @blur="buffValueChange" />
        <n-input-number class="pl-1" v-model:value="item.level" :min="1" :max="3" />
        <n-button style="margin-left: 12px" @click="removeItem(index)">
          删除
        </n-button>
      </n-form-item>
      <n-form-item>
        <n-space>
          <n-button attr-type="button" @click="addItem">
            增加
          </n-button>
          <n-button attr-type="button" @click="handleValidateClick">
            确认设定
          </n-button>
        </n-space>
      </n-form-item>
    </n-form>
  </div>
</template>

<style scoped></style>
