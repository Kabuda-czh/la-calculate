<script setup lang="ts">
import { NInputNumber, NSelect, NTag, type DataTableColumns } from "naive-ui";
import { accessoryMap, classesWithBuffOptionsMap } from "../config"
import { calculate_build } from "../utils";
// import { invoke } from "@tauri-apps/api/tauri";

const props = withDefaults(defineProps<{
  calculatePageParam: Calculate.CalculatePageParam
}>(), {
  calculatePageParam: () => {
    return {
      need_builds: [] as Calculate.Build[],
      stone_builds: {} as Calculate.StoneBuild,
      self_builds: {} as Calculate.SelfBuild,
    }
  }
})

const emit = defineEmits<{
  settingSuccess: [calculate_price_params: Calculate.CalculatePriceParam[], result_array: Calculate.CalculateResult["result_array"]]
  goToPrev: []
}>()

const loading = ref<boolean>(false)
const calculateResult = ref<Calculate.CalculateResult>({
  result_array: [],
  total_used_accessory_array: []
})
const data = ref<Calculate.CalculatePriceParam[]>([])

const columns: DataTableColumns<Calculate.CalculatePriceParam> = [
  {
    title: "首饰位置",
    key: "accessory_name",
  },
  {
    title: "刻印",
    key: "build_string",
  },
  {
    title: "是否为遗物级别",
    key: "isArtifact",
    render: (row) => {
      // return h(NTag, {
      //   bordered: false,
      //   type: row.isArtifact ? "success" : "error",
      //   size: "small"
      // }, {
      //   default: () => row.isArtifact ? "是" : "否"
      // })
      return h(NSelect, {
        size: "small",
        value: row.is_artifact ? "是" : "否",
        disabled: !row.is_artifact_disabled,
        options: [
          { label: "是", value: 1 },
          { label: "否", value: 0 }
        ],
        onUpdateValue(v) {
          row.is_artifact = v
        }
      })
    }
  },
  {
    title: "价格(注意: 价格为0时不会计算在内)",
    key: "price",
    titleAlign: "center",
    render: (row) => {
      return h(NInputNumber, {
        showButton: false,
        value: row.price,
        onUpdateValue(v) {
          row.price = v as number
        }
      })
    }
  }
]

async function calculate() {
  loading.value = true
  // calculateResult.value = await invoke("build_calculate", { buildParam: props.calculatePageParam })
  calculateResult.value = await calculate_build(props.calculatePageParam)
  loading.value = false

  data.value = calculateResult.value.total_used_accessory_array.sort().map((buildString) => {
    return {
      accessory: buildString.split(":")[0],
      accessory_name: accessoryMap[buildString.split(":")[0]],
      build: buildString.split(":")[1].split(",").reduce((acc, cur) => {
        acc[cur.split("-")[0]] = Number(cur.split("-")[1])
        return acc
      }, {} as Record<string, number>),
      build_string: buildString.split(":")[1].split(",").map((build) => {
        return classesWithBuffOptionsMap[build.split("-")[0]] + " " + build.split("-")[1]
      }).join("\t"),
      is_artifact: 0,
      is_artifact_disabled: buildString.indexOf("5") !== -1 ? true : false,
      price: 0,
      // price: Math.floor(Math.random() * 100000),
      base_string: buildString
    }
  })
}

const settingClick = async () => {
  // console.log(data.value, calculateResult.value.result_array)
  // console.log(await calculate_price([data.value, calculateResult.value.result_array]))
  emit("settingSuccess", data.value, calculateResult.value.result_array)
}

const goToPrev = () => {
  data.value = []
  calculateResult.value = {
    result_array: [],
    total_used_accessory_array: []
  }
  emit('goToPrev')
}
</script>

<template>
  <div>
    <n-list hoverable clickable class="mb-5">
      <n-list-item>
        <n-thing title="需求刻印" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <n-tag v-for="build in props.calculatePageParam.need_builds" :key="build.code" :bordered="false"
                type="info" size="small">
                {{ classesWithBuffOptionsMap[build.code] }} {{ build.level }}
              </n-tag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="设定的能力石" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <n-tag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.stone_builds?.buff_1?.code] }} {{
                  props.calculatePageParam.stone_builds?.buff_1?.value }}
              </n-tag>
              <n-tag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.stone_builds?.buff_2?.code] }} {{
                  props.calculatePageParam.stone_builds?.buff_2?.value }}
              </n-tag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="设定的自身刻印" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <n-tag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.self_builds?.buff_1?.code] }} {{
                  props.calculatePageParam.self_builds?.buff_1?.value }}
              </n-tag>
              <n-tag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.self_builds?.buff_2?.code] }} {{
                  props.calculatePageParam.self_builds?.buff_2?.value }}
              </n-tag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
    </n-list>

    <n-button class="mb-5" @click="calculate"> 开始计算 </n-button>

    <n-spin :show="loading">
      <n-data-table :columns="columns" :data="data" :pagination="{ pageSize: 10 }" />
      <template #description>
        正在计算中......
      </template>
    </n-spin>

    <div>
      <n-button attr-type="button" @click="goToPrev">
      返回上一级
    </n-button>
      <n-button class="ml-1" attr-type="button" @click="settingClick">
        确认设置, 开始计算
      </n-button>
    </div>
  </div>
</template>

<style scoped></style>
