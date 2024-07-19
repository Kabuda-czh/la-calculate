<script setup lang="tsx">
import { type DataTableColumns, NInput, NInputNumber, NSelect, NSwitch, NTag, NText } from 'naive-ui'
import type { CSSProperties } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { accessoryMap, classesWithBuffOptionsMap } from '../config'

const props = withDefaults(defineProps<{
  calculatePageParam: Calculate.CalculatePageParam
}>(), {
  calculatePageParam: () => {
    return {
      need_builds: [] as Calculate.Build[],
      stone_builds: {} as Calculate.StoneBuild,
      self_builds: {} as Calculate.SelfBuild,
    }
  },
})

const emit = defineEmits<{
  settingSuccess: [calculate_price_params: Calculate.CalculatePriceParam[], result_array: Calculate.CalculateResult['result_array']]
  goToPrev: []
}>()

const loading = ref<boolean>(false)
const calculateResult = ref<Calculate.CalculateResult>({
  result_array: [],
  total_used_accessory_array: [],
})
const data = ref<Calculate.CalculatePriceParam[]>([])
const artifact_check = ref<boolean>(false)
const firstCalculate = ref<boolean>(true)

function rowDisabledInputComputed(rowData: Calculate.CalculatePriceParam) {
  const accessoryNameCountMap = {
    amulet: 1,
    earring: 2,
    ring: 2,
  }

  return computed(() => {
    return data.value.filter(
      row =>
        row.accessory === rowData.accessory
        && row.is_buy
        && row.base_string !== rowData.base_string,
    ).length >= accessoryNameCountMap[rowData.accessory]
  })
}

const columns: DataTableColumns<Calculate.CalculatePriceParam> = [
  {
    title: '首饰位置',
    key: 'accessory_name',
    filterMultiple: false,
    filterOptions: Object.keys(accessoryMap).map((key) => {
      return {
        label: accessoryMap[key],
        value: key,
      }
    }),
    filter: (value, row) => {
      return !!~row.accessory.indexOf(value.toString())
    },
  },
  {
    title: '刻印',
    key: 'build_string',
    filterOptions: [],
    filterMode: 'and',
    filter: (value, row) => {
      return !!~row.build_string.indexOf(value.toString())
    },
  },
  {
    title: '是否为遗物级别',
    key: 'isArtifact',
    render: (row) => {
      return (
        <>
          <NSelect
            size="small"
            value={row.is_artifact ? '是' : '否'}
            disabled={!row.is_artifact_disabled || rowDisabledInputComputed(row).value}
            options={[
              { label: '是', value: 1 },
              { label: '否', value: 0 },
            ]}
            onUpdateValue={(v) => {
              row.is_artifact = v
            }}
          />
        </>
      )
    },
  },
  {
    title: '是否已经购买',
    key: 'isBuy',
    render: (row) => {
      return (
        <>
          <NSwitch
            size="small"
            value={row.is_buy}
            disabled={rowDisabledInputComputed(row).value}
            onUpdateValue={(v) => {
              row.is_buy = v
            }}
          />
        </>
      )
    },
  },
  {
    title: '价格(注意: 价格为0时不会计算在内)',
    key: 'price',
    titleAlign: 'center',
    render: (row) => {
      return (
        <>
          <NInputNumber
            showButton={false}
            value={row.price}
            disabled={rowDisabledInputComputed(row).value}
            onUpdateValue={(v) => {
              row.price = v || 0
            }}
          />
        </>
      )
    },
  },
  {
    title: '备注',
    key: 'remark',
    render: (row) => {
      return (
        <>
          <NInput
            value={row.remark}
            disabled={rowDisabledInputComputed(row).value}
            onUpdateValue={(v) => {
              row.remark = v
            }}
          />
        </>
      )
    },
  },
]

function railStyle({
  focused,
  checked,
}: {
  focused: boolean
  checked: boolean
}) {
  const style: CSSProperties = {}
  if (checked) {
    style.background = '#FA5D00'
    if (focused) {
      style.boxShadow = '0 0 0 2px #FA5D0040'
    }
  }
  else {
    style.background = '#E3C7A1'
    if (focused) {
      style.boxShadow = '0 0 0 2px #E3C7A140'
    }
  }
  return style
}

async function calculate() {
  loading.value = true
  calculateResult.value = await invoke('calculate_build', { buildParam: props.calculatePageParam, artifactCheck: artifact_check.value })
  loading.value = false
  firstCalculate.value = false

  const buildStringSet = new Set<string>()

  data.value = calculateResult.value.total_used_accessory_array.sort().map((buildString) => {
    return {
      accessory: buildString.split(':')[0] as 'amulet' | 'earring' | 'ring',
      accessory_name: accessoryMap[buildString.split(':')[0]],
      build: buildString.split(':')[1].split(',').reduce((acc, cur) => {
        acc[cur.split('-')[0]] = Number(cur.split('-')[1])
        return acc
      }, {} as Record<string, number>),
      build_string: buildString.split(':')[1].split(',').map((build) => {
        const build_name = classesWithBuffOptionsMap[build.split('-')[0]]
        buildStringSet.add(build_name)
        return `${build_name} ${build.split('-')[1]}`
      }).join('\t'),
      is_artifact: artifact_check.value ? 1 : 0,
      is_artifact_disabled: artifact_check.value ? false : !buildString.includes('6'),
      is_buy: false,
      price: 0,
      base_string: buildString,
      remark: '',
    }
  })

  filtersUpdate(buildStringSet)
}

function setFilterOption() {
  const buildStringSet = new Set<string>()
  data.value.forEach((row) => {
    buildStringSet.add(row.build_string.split(' ')[0])
  })
  filtersUpdate(buildStringSet)
}

function filtersChange(filterMap: {
  accessory_name: string
  build_string: string[]
}) {
  const { accessory_name, build_string } = filterMap

  const accessoryMessage = accessory_name
    ? `当前首饰过滤为: ${accessoryMap[accessory_name]}`
    : '当前首饰过滤为: 无'

  const buildStringMessage = build_string?.length
    ? `当前刻印过滤为: ${build_string.join(',')}`
    : '当前刻印过滤为: 无'

  window.$notification?.success({
    title: '过滤条件',
    content: () => (
      <div>
        <p>{accessoryMessage}</p>
        <p>{buildStringMessage}</p>
      </div>
    ),
    duration: 2000,
    closable: false,
  })
}

function filtersFunction(filtersValue: any) {
  if (filtersValue instanceof Set)
    filtersUpdate(filtersValue)
  else
    filtersChange(filtersValue)
}

function filtersUpdate(buildStringSet: Set<string>) {
  columns[1].filterOptions = Array.from(buildStringSet).map((buildString) => {
    return {
      label: buildString,
      value: buildString,
    }
  })
}

async function settingClick() {
  const accessoryCount = data.value.reduce((acc, row) => {
    if (row?.price && row.price !== 0) {
      if (row.accessory === 'amulet')
        acc.amulet++
      else if (row.accessory === 'earring')
        acc.earring++
      else if (row.accessory === 'ring')
        acc.ring++
    }
    return acc
  }, { amulet: 0, earring: 0, ring: 0 })

  const { amulet, earring, ring } = accessoryCount

  if (amulet < 1 || earring < 2 || ring < 2) {
    window.$dialog?.warning({
      title: '提示',
      content: () => (
        <div>
          <p>请至少设置 1 个项链, 2 个耳环, 2 个戒指</p>
          <div class="flex gap-2">
            <NText>当前设置: </NText>
            <NText type={amulet < 1 ? 'error' : 'success'}>
              项链:
              {amulet}
              个
            </NText>
            <NText type={earring < 2 ? 'error' : 'success'}>
              耳环:
              {earring}
              个
            </NText>
            <NText type={ring < 2 ? 'error' : 'success'}>
              戒指:
              {ring}
              个
            </NText>
          </div>
        </div>
      ),
    })
    return
  }

  emit('settingSuccess', data.value, calculateResult.value.result_array)
}

function goToPrev() {
  data.value = []
  calculateResult.value = {
    result_array: [],
    total_used_accessory_array: [],
  }
  artifact_check.value = false
  firstCalculate.value = true
  columns[1].filterOptions = []
  emit('goToPrev')
}

defineExpose({
  data,
  setFilterOption,
  calculateResult,
  artifact_check,
  firstCalculate,
})
</script>

<template>
  <div>
    <n-list hoverable clickable class="mb-5">
      <n-list-item>
        <n-thing title="需求刻印" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <NTag
                v-for="build in props.calculatePageParam.need_builds" :key="build.code" :bordered="false"
                type="info" size="small"
              >
                {{ classesWithBuffOptionsMap[build.code] }} {{ build.level }}
              </NTag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="设定的能力石" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <NTag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.stone_builds?.buff_1?.code] }} {{
                  props.calculatePageParam.stone_builds?.buff_1?.value }}
              </NTag>
              <NTag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.stone_builds?.buff_2?.code] }} {{
                  props.calculatePageParam.stone_builds?.buff_2?.value }}
              </NTag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
      <n-list-item>
        <n-thing title="设定的自身刻印" content-style="margin-top: 10px;">
          <template #description>
            <n-space size="small" style="margin-top: 4px">
              <NTag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.self_builds?.buff_1?.code] }} {{
                  props.calculatePageParam.self_builds?.buff_1?.value }}
              </NTag>
              <NTag :bordered="false" type="info" size="small">
                {{ classesWithBuffOptionsMap[props.calculatePageParam.self_builds?.buff_2?.code] }} {{
                  props.calculatePageParam.self_builds?.buff_2?.value }}
              </NTag>
            </n-space>
          </template>
        </n-thing>
      </n-list-item>
    </n-list>

    <div class="mb-5 w-full flex justify-between items-center">
      <NSwitch v-model:value="artifact_check" :round="false" :rail-style="railStyle">
        <template #checked>
          当前仅处理遗物首饰
        </template>
        <template #unchecked>
          当前处理古代和遗物首饰
        </template>
      </NSwitch>
      <n-button @click="calculate">
        开始计算
      </n-button>
    </div>

    <n-spin :show="loading">
      <n-data-table :columns="columns" :data="data" :pagination="{ pageSize: 10 }" @update:filters="filtersFunction" />
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
