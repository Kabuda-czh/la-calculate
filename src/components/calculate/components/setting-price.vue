<script setup lang="ts">
import { NInputNumber, NSelect, NTag, NText, type DataTableColumns } from "naive-ui";
import { accessoryMap, classesWithBuffOptionsMap } from "../config"
import { calculate_build } from "../utils";
import { CSSProperties } from "vue";
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
const artifact_check = ref<boolean>(false)
const firstCalculate = ref<boolean>(true)

const columns: DataTableColumns<Calculate.CalculatePriceParam> = [
  {
    title: "首饰位置",
    key: "accessory_name",
    filterMultiple: false,
    filterOptions: Object.keys(accessoryMap).map((key) => {
      return {
        label: accessoryMap[key],
        value: key
      }
    }),
    filter: (value, row) => {
      return !!~row.accessory.indexOf(value.toString())
    }
  },
  {
    title: "刻印",
    key: "build_string",
  },
  {
    title: "是否为遗物级别",
    key: "isArtifact",
    render: (row) => {
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

const railStyle = ({
  focused,
  checked
}: {
  focused: boolean
  checked: boolean
}) => {
  const style: CSSProperties = {}
  if (checked) {
    style.background = '#FA5D00'
    if (focused) {
      style.boxShadow = '0 0 0 2px #FA5D0040'
    }
  } else {
    style.background = '#E3C7A1'
    if (focused) {
      style.boxShadow = '0 0 0 2px #E3C7A140'
    }
  }
  return style
}

async function calculate() {
  loading.value = true
  // calculateResult.value = await invoke("build_calculate", { buildParam: props.calculatePageParam })
  calculateResult.value = await calculate_build(props.calculatePageParam, artifact_check.value)
  loading.value = false
  firstCalculate.value = false

  data.value = calculateResult.value.total_used_accessory_array.sort().map((buildString) => {
    return {
      accessory: buildString.split(":")[0],
      accessory_name: accessoryMap[buildString.split(":")[0]],
      build: buildString.split(":")[1].split(",").reduce((acc, cur) => {
        acc[cur.split("-")[0]] = Number(cur.split("-")[1])
        return acc
      }, {} as Record<string, number>),
      build_string: buildString.split(":")[1].split(",").map((build) => {
        const build_name = classesWithBuffOptionsMap[build.split("-")[0]]
        return build_name+ " " + build.split("-")[1]
      }).join("\t"),
      is_artifact: artifact_check.value ? 1 : 0,
      is_artifact_disabled: artifact_check.value ? false : buildString.indexOf("6") !== -1 ? false : true,
      price: 0,
      base_string: buildString
    }
  })
}

const settingClick = async () => {
  const accessoryCount = data.value.reduce((acc, row) => {
    if (row.accessory === "Amulet") {
      acc.amulet = acc.amulet + 1
    } else if (row.accessory === "Earring") {
      acc.earring = acc.earring + 1
    } else if (row.accessory === "Ring") {
      acc.ring = acc.ring + 1
    }
    return acc
  }, { amulet: 0, earring: 0, ring: 0 })

  if (accessoryCount.amulet < 1 || accessoryCount.earring < 2 || accessoryCount.ring < 2) {
    window.$dialog?.warning({
      title: "提示",
      content: () => h("div", [
        h("p", "请至少设置 1 个项链, 2 个耳环, 2 个戒指"),
        h("div", { class: 'flex gap-2' }, [
          h(NText, null, { default: () => "当前设置: " }),
          h(NText, { type: accessoryCount.amulet < 1 ? 'error' : 'success', }, { default: () => `项链: ${accessoryCount.amulet}个` }),
          h(NText, { type: accessoryCount.earring < 2 ? 'error' : 'success' }, { default: () => `耳环: ${accessoryCount.earring}个` }),
          h(NText, { type: accessoryCount.ring < 2 ? 'error' : 'success' }, { default: () => `戒指: ${accessoryCount.ring}个` }),
        ]),
      ])
    })
    return
  }

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

    <div class="mb-5 w-full flex justify-between items-center">
      <n-switch v-model:value="artifact_check" :round="false" :rail-style="railStyle">
        <template #checked>
          当前仅处理遗物首饰
        </template>
        <template #unchecked>
          当前处理古代和遗物首饰
        </template>
      </n-switch>
      <n-button @click="calculate"> 开始计算 </n-button>
    </div>

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
      <n-button class="ml-1" attr-type="button" :disabled="firstCalculate" @click="settingClick">
        确认设置, 开始计算
      </n-button>
    </div>
  </div>
</template>

<style scoped></style>
