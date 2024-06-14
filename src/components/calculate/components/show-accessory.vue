<script setup lang="tsx">
import { type DataTableColumns, NNumberAnimation, NTag } from 'naive-ui'
import { invoke } from '@tauri-apps/api/tauri'

const props = withDefaults(defineProps<{
  calculatePriceParam: [Calculate.CalculatePriceParam[], Calculate.CalculateResult['result_array']]
}>(), {
  calculatePriceParam: () => {
    return [
      [],
      [],
    ]
  },
})

const emit = defineEmits<{
  goToPrev: []
}>()

const loading = ref<boolean>(false)
const sliderValueArray = ref<[number, number]>([0, 0])
const sliderMin = ref<number>(0)
const sliderMax = ref<number>(0)
const data = ref<Calculate.CalculatePriceResultTableColumn[]>([])
const copyData = ref<Calculate.CalculatePriceResultTableColumn[]>([])
const calculatePriceResult = ref<Calculate.CalculatePriceResult[]>([])

function createAccessoryColumnRender(item: Calculate.CalculatePriceParam) {
  return (
    <div>
      <span>{item.build_string}</span>
      <br />
      <NTag type={item.is_artifact ? 'error' : 'warning'} bordered={false}>
        {item.is_artifact ? '遗物' : '古代'}
      </NTag>
      <br />
      <NNumberAnimation showSeparator from={0} to={item.price} />
      <br />
      <span>
        备注:
        {item.remark || ''}
      </span>
    </div>
  )
}

const columns: DataTableColumns<Calculate.CalculatePriceResultTableColumn> = [
  {
    title: '项链',
    key: 'amulet',
    render: row => createAccessoryColumnRender(row.amulet),
  },
  {
    title: '耳环',
    key: 'earring_1',
    render: row => createAccessoryColumnRender(row.earring_1),
  },
  {
    title: '耳环',
    key: 'earring_2',
    render: row => createAccessoryColumnRender(row.earring_2),
  },
  {
    title: '戒指',
    key: 'ring_1',
    render: row => createAccessoryColumnRender(row.ring_1),
  },
  {
    title: '戒指',
    key: 'ring_2',
    render: row => createAccessoryColumnRender(row.ring_2),
  },
  {
    title: '总价格',
    key: 'price_total',
    render: (row) => {
      return <NNumberAnimation showSeparator from={0} to={row.price_total} />
    },
  },
]

async function calculate() {
  loading.value = true
  calculatePriceResult.value = await invoke('calculate_price', { param: props.calculatePriceParam })
  loading.value = false

  data.value = calculatePriceResult.value.map((result) => {
    const omitPriceTotalData = result.used_items.reduce((acc, item) => {
      if (item.accessory === 'Amulet') {
        acc.amulet = item
      }

      if (item.accessory === 'Earring') {
        if (acc.earring_1) {
          acc.earring_2 = item
        }
        else {
          acc.earring_1 = item
        }
      }

      if (item.accessory === 'Ring') {
        if (acc.ring_1) {
          acc.ring_2 = item
        }
        else {
          acc.ring_1 = item
        }
      }

      return acc
    }, {} as Omit<Calculate.CalculatePriceResultTableColumn, 'price_total'>)

    return {
      ...omitPriceTotalData,
      price_total: result.price_total,
    }
  })

  const [min, max] = [data.value[0]?.price_total || 0, data.value?.[data.value.length - 1]?.price_total || 0]

  sliderMin.value = data.value[0]?.price_total || 0
  sliderMax.value = data.value?.[data.value.length - 1]?.price_total || 0
  sliderValueArray.value = [min, max]

  copyData.value = JSON.parse(JSON.stringify(data.value))
}

function sliderDragend() {
  const [min, max] = sliderValueArray.value
  data.value = copyData.value.filter(item => item.price_total >= min && item.price_total <= max)
}

function goToPrev() {
  loading.value = true
  data.value = []
  calculatePriceResult.value = []
  emit('goToPrev')
}

defineExpose({
  calculate,
})
</script>

<template>
  <div>
    <n-space vertical>
      <div class="flex items-center gap-5">
        <p>表格中首饰的价格区间: </p>
        <n-text type="success">
          {{ sliderMin }}
        </n-text>
        <p>~</p>
        <n-text type="error">
          {{ sliderMax }}
        </n-text>
      </div>
      <n-slider
        v-model:value="sliderValueArray" range :step="1" :min="sliderMin" :max="sliderMax"
        @dragend="sliderDragend"
      />
      <n-space>
        <div class="flex items-center gap-5">
          <p>手动输入区间</p>
          <n-input-number
            v-model:value="sliderValueArray[0]" size="small" :min="sliderMin" :max="sliderMax"
            :show-button="false" @blur="sliderDragend"
          />
          <p>~</p>
          <n-input-number
            v-model:value="sliderValueArray[1]" size="small" :min="sliderMin" :max="sliderMax"
            :show-button="false" @blur="sliderDragend"
          />
        </div>
      </n-space>
    </n-space>

    <n-spin :show="loading">
      <n-data-table class="mt-5" :columns="columns" :data="data" :pagination="{ pageSize: 4 }" />
    </n-spin>

    <n-button attr-type="button" @click="goToPrev">
      返回上一级
    </n-button>
  </div>
</template>

<style scoped></style>
