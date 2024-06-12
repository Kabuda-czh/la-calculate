<script setup lang="ts">
import { NNumberAnimation, NTag, type DataTableColumns } from "naive-ui";
import { calculate_price } from "../utils";
// import { invoke } from "@tauri-apps/api/tauri";

const props = withDefaults(defineProps<{
  calculatePriceParam: [Calculate.CalculatePriceParam[], Calculate.CalculateResult["result_array"]]
}>(), {
  calculatePriceParam: () => {
    return [
      [], []
    ]
  }
})

const emit = defineEmits<{
  goToPrev: []
}>()

const loading = ref<boolean>(true)
const data = ref<Calculate.CalculatePriceResultTableColumn[]>([])
const calculatePriceResult = ref<Calculate.CalculatePriceResult[]>([])

const createAccessoryColumnRender = (item: Calculate.CalculatePriceParam) => {
  return h('div', [
    h('span', item.build_string),
    h('br'),
    h(
      NTag,
      {
        type: item.is_artifact ? "error" : "warning",
        bordered: false,
      },
      item.is_artifact ? '遗物' : '古代'
    ),
    h('br'),
    h(NNumberAnimation, {
      showSeparator: true,
      from: 0,
      to: item.price
    })
  ])
}

const columns: DataTableColumns<Calculate.CalculatePriceResultTableColumn> = [
  {
    title: "项链",
    key: "amulet",
    render: (row) => createAccessoryColumnRender(row.amulet)
  },
  {
    title: "耳环",
    key: "earring_1",
    render: (row) => createAccessoryColumnRender(row.earring_1)
  },
  {
    title: "耳环",
    key: "earring_2",
    render: (row) => createAccessoryColumnRender(row.earring_2)
  },
  {
    title: "戒指",
    key: "ring_1",
    render: (row) => createAccessoryColumnRender(row.ring_1)
  },
  {
    title: "戒指",
    key: "ring_2",
    render: (row) => createAccessoryColumnRender(row.ring_2)
  },
  {
    title: "总价格",
    key: "price_total",
    render: (row) => {
      return h(NNumberAnimation, {
        showSeparator: true,
        from: 0,
        to: row.price_total,
      })
    }
  }
]

async function calculate() {
  // calculateResult.value = await invoke("build_calculate", { buildParam: props.calculatePageParam })
  calculatePriceResult.value = await calculate_price(props.calculatePriceParam)
  loading.value = false

  data.value = calculatePriceResult.value.map((result) => {
    const omitPriceTotalData = result.used_items.reduce((acc, item) => {
      if (item.accessory === "Amulet") {
        acc["amulet"] = item
      }

      if (item.accessory === "Earring") {
        if (acc["earring_1"]) {
          acc["earring_2"] = item
        } else {
          acc["earring_1"] = item
        }
      }

      if (item.accessory === "Ring") {
        if (acc["ring_1"]) {
          acc["ring_2"] = item
        } else {
          acc["ring_1"] = item
        }
      }

      return acc
    }, {} as Omit<Calculate.CalculatePriceResultTableColumn, "price_total">)

    return {
      ...omitPriceTotalData,
      price_total: result.price_total
    }
  })
}

const goToPrev = () => {
  loading.value = true
  data.value = []
  calculatePriceResult.value = []
  emit('goToPrev')
}

defineExpose({
  calculate
})
</script>

<template>
  <div>
    <n-data-table :columns="columns" :data="data" :pagination="{ pageSize: 4 }" />

    <n-button attr-type="button" @click="goToPrev">
      返回上一级
    </n-button>
  </div>
</template>

<style scoped></style>
