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

  if (sumBuildsLevel > 89) {
    window.$message?.warning('刻印等级总和不能超过当前版本最大值 「89」 ！')
    return
  }

  formRef.value?.validate((errors) => {
    if (!errors) {
      const buildDescription = dynamicForm.builds.map(item => {
        return `${classesWithBuffOptions.find(option => option.value === item.code)?.label}${item.level}`
      }).join(' ')
      window.$message?.success(`刻印设定成功: ${buildDescription}`)
      emit('buildSuccess', dynamicForm.builds)
    } else {
      window.$message?.warning('请完善刻印设定')
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
    <n-alert class="mb-5" title="提示" type="warning">
      在下列选择你想要的刻印, 并设置对应等级, 注意不要选择当前版本无法达到的刻印以及数量
    </n-alert>
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
