interface BuildItem {
  code: string
  level: 3 | 1 | 2
  value: number
}

const usedAccessoryNameArray = ['amulet', 'earring_1', 'earring_2', 'ring_1', 'ring_2']

function calculate_build(buildParam: Calculate.CalculatePageParam, artifactCheck: boolean = false): Promise<Calculate.CalculateResult> {
  return new Promise((resolve) => {
    const engravingPointDistributions = [
      [4, 3],
      [5, 3],
      [6, 3],
    ]

    if (artifactCheck)
      engravingPointDistributions.pop()

    const resultArray = new Set<string>()
    const totalUsedAccessorySet = new Set<string>()
    const buildItems = buildParam.need_builds.map(build => ({
      code: build.code,
      level: build.level,
      value: 0,
    }))

    function addSettingValue(
      name: string,
      buffValue: {
        code: string
        value: number
      },
      buildItems: BuildItem[],
      elementUsedMap: Record<string, { code: string, value: number }[]>,
    ) {
      const item = buildItems.find(element => element.code === buffValue.code)
      if (item) {
        if (!elementUsedMap[name])
          elementUsedMap[name] = []
        if (elementUsedMap[name].length < 2) {
          elementUsedMap[name].push({ code: item.code, value: buffValue.value })
          item.value += buffValue.value
        }
      }
    }

    function isValid(buildItems: BuildItem[]) {
      return buildItems.every(item => item.value >= item.level * 5 && item.value < (item.level + 1) * 5)
    }

    function tryCombination(index: number, buildItems: BuildItem[], elementUsedMap: Record<string, { code: string, value: number }[]>) {
      if (index === 5) {
        if (isValid(buildItems)) {
          const { amulet } = elementUsedMap
          let { earring_1, earring_2, ring_1, ring_2 } = elementUsedMap

          // 函数计算数组中元素的和
          const calculateSum = (array: { code: string, value: number }[]) =>
            array.reduce((acc, cur) => acc + Number(cur.code) + cur.value, 0)

          // 交换数组
          const swapIfNecessary = (sum1: number, sum2: number, array1: any[], array2: any[]) => {
            if (sum1 < sum2) {
              return [array2, array1]
            }
            return [array1, array2]
          }

          const earring_1_sum = calculateSum(earring_1)
          const earring_2_sum = calculateSum(earring_2)
          const ring_1_sum = calculateSum(ring_1)
          const ring_2_sum = calculateSum(ring_2)

          ;[earring_1, earring_2] = swapIfNecessary(earring_1_sum, earring_2_sum, earring_1, earring_2)
          ;[ring_1, ring_2] = swapIfNecessary(ring_1_sum, ring_2_sum, ring_1, ring_2)

          const sortAndMap = (arr: { code: string, value: number }[], label: string) =>
            `${label}:${arr.sort((a, b) => +a.code - +b.code).map(e => `${e.code}-${e.value}`).join(',')}`

          const sortFilteredElementUsedArray = [
            sortAndMap(amulet, 'amulet'),
            sortAndMap(earring_1, 'earring'),
            sortAndMap(earring_2, 'earring'),
            sortAndMap(ring_1, 'ring'),
            sortAndMap(ring_2, 'ring'),
          ]

          const sortFilteredElementUsedString = sortFilteredElementUsedArray.join('|')

          if (!resultArray.has(sortFilteredElementUsedString)) {
            resultArray.add(sortFilteredElementUsedString)
            sortFilteredElementUsedArray.forEach(e => totalUsedAccessorySet.add(e))
            return
          }
        }
        return
      }

      const points = engravingPointDistributions

      for (let i = 0; i < points.length; i++) {
        const pointPair = points[i]

        for (let j = 0; j < buildItems.length; j++) {
          const item1 = buildItems[j]
          if (item1.value < item1.level * 5 && !elementUsedMap[usedAccessoryNameArray[index]]?.find(e => e.code === item1.code)) {
            item1.value += pointPair[0]
            if (!elementUsedMap[usedAccessoryNameArray[index]])
              elementUsedMap[usedAccessoryNameArray[index]] = []
            elementUsedMap[usedAccessoryNameArray[index]].push({ code: item1.code, value: pointPair[0] })

            for (let k = 0; k < buildItems.length; k++) {
              if (k !== j) {
                const item2 = buildItems[k]
                if (item2.value < item2.level * 5 && !elementUsedMap[usedAccessoryNameArray[index]]?.find(e => e.code === item2.code)) {
                  item2.value += pointPair[1]
                  elementUsedMap[usedAccessoryNameArray[index]].push({ code: item2.code, value: pointPair[1] })

                  tryCombination(index + 1, buildItems, elementUsedMap)

                  item2.value -= pointPair[1]
                  elementUsedMap[usedAccessoryNameArray[index]].splice(elementUsedMap[usedAccessoryNameArray[index]].findIndex(e => e.code === item2.code), 1)
                }
              }
            }

            item1.value -= pointPair[0]
            elementUsedMap[usedAccessoryNameArray[index]].splice(elementUsedMap[usedAccessoryNameArray[index]].findIndex(e => e.code === item1.code), 1)
          }
        }
      }
    }

    const elementUsedMap = {}
    addSettingValue('stone_1', buildParam.stone_builds.buff_1, buildItems, elementUsedMap)
    addSettingValue('stone_2', buildParam.stone_builds.buff_2, buildItems, elementUsedMap)
    addSettingValue('self_1', buildParam.self_builds.buff_1, buildItems, elementUsedMap)
    addSettingValue('self_2', buildParam.self_builds.buff_2, buildItems, elementUsedMap)

    tryCombination(0, buildItems, elementUsedMap)

    resolve({
      result_array: Array.from(resultArray),
      total_used_accessory_array: Array.from(totalUsedAccessorySet).sort(),
    })
  })
}

function calculate_price(param: [Calculate.CalculatePriceParam[], string[]]): Promise<Calculate.CalculatePriceResult[]> {
  return new Promise((resolve) => {
    const [price_array, items_array] = param

    let amuletArray = price_array.filter(e => e.accessory === 'amulet')
    let earringArray = price_array.filter(e => e.accessory === 'earring')
    let ringArray = price_array.filter(e => e.accessory === 'ring')

    const earringBuyOne: Calculate.CalculatePriceParam[] = []
    const ringBuyOne: Calculate.CalculatePriceParam[] = []

    const isBuyAmuletCount = amuletArray.filter(e => e.is_buy).length
    const isBuyEarringCount = earringArray.filter(e => e.is_buy).length
    const isBuyRingCount = ringArray.filter(e => e.is_buy).length

    if (isBuyAmuletCount === 1)
      amuletArray = amuletArray.filter(e => e.is_buy)
    if (isBuyEarringCount === 1)
      earringBuyOne.push(earringArray.filter(e => e.is_buy)[0])
    else if (isBuyEarringCount === 2)
      earringArray = earringArray.filter(e => e.is_buy)
    if (isBuyRingCount === 1)
      ringBuyOne.push(ringArray.filter(e => e.is_buy)[0])
    else if (isBuyRingCount === 2)
      ringArray = ringArray.filter(e => e.is_buy)

    const price_array_filtered = [...amuletArray, ...earringArray, ...ringArray].filter(e => e.price !== 0 || e.is_buy)

    function isInvalidAccessory(items: string[], accessoryArray: Calculate.CalculatePriceParam[]) {
      if (accessoryArray.length === 1) {
        const baseString = accessoryArray[0].base_string
        const count = items.filter(item => item === baseString).length
        return !items.includes(baseString) || count > 1
      }
      return false
    }

    const items_array_filtered = items_array.filter((items) => {
      const items_array = items.split('|')
      return items_array.every((item) => {
        const [accessory] = item.split(':')
        return price_array_filtered.find(e =>
          e.accessory === accessory && e.base_string === item,
        )
      })
    })

    const items_price_array = items_array_filtered.reduce((acc, cur) => {
      const items = cur.split('|')

      if (isInvalidAccessory(items, earringBuyOne) || isInvalidAccessory(items, ringBuyOne))
        return acc

      function parseItem(item: string) {
        const [accessory] = item.split(':')
        return {
          accessory,
          buffs: item,
        }
      }

      function findBuild(parsedItem: ReturnType<typeof parseItem>) {
        return price_array_filtered.find(e =>
          e.accessory === parsedItem.accessory && e.base_string === parsedItem.buffs,
        )
      }

      const valid_items = items.every((item) => {
        const parsedItem = parseItem(item)
        const build = findBuild(parsedItem)
        return build && (build.price !== 0 || build.is_buy)
      })

      if (valid_items) {
        const used_items = items.map((item) => {
          const parsedItem = parseItem(item)
          const build = findBuild(parsedItem)!
          return {
            accessory_name: parsedItem.accessory,
            accessory: build.accessory,
            build_string: build.build_string,
            build: build.build,
            is_artifact: build.is_artifact,
            is_artifact_disabled: build.is_artifact_disabled,
            is_buy: build.is_buy,
            price: build.price,
            base_string: build.base_string,
            remark: build.remark,
          } as Calculate.CalculatePriceParam
        })

        const price_total = used_items.reduce((acc, item) => acc + item.price, 0)

        acc.push({ used_items, price_total })
      }

      return acc
    }, [] as Calculate.CalculatePriceResult[])

    resolve(items_price_array.filter(item => item.price_total > 0).sort((a, b) => a.price_total - b.price_total))
  })
}

export {
  calculate_build,
  calculate_price,
}
