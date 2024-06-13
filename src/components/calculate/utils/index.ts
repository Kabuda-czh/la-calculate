interface BuildItem {
  code: string | number
  level: 3 | 1 | 2
  value: number
}

const engravingPointDistributions = [
  [4, 3],
  [5, 3],
  [6, 3]
];

const usedAccessoryNameArray = ['Amulet', 'Earring_1', 'Earring_2', 'Ring_1', 'Ring_2'];

function calculate_build(buildParam: Calculate.CalculatePageParam, artifactCheck: boolean = false): Promise<Calculate.CalculateResult> {
  return new Promise((resolve) => {
    if (artifactCheck)
      engravingPointDistributions.pop();

    const resultArray = new Set<string>();
    const totalUsedAccessorySet = new Set<string>();
    const buildItems = buildParam.need_builds.map(build => ({
      code: build.code,
      level: build.level,
      value: 0
    }));

    function addSettingValue(
      name: string,
      buffValue: Calculate.StoneBuild['buff_1'] | Calculate.SelfBuild['buff_1'],
      buildItems: BuildItem[],
      elementUsedMap: Record<string, { code: string | number; value: number }[]>
    ) {
      const item = buildItems.find(element => element.code === buffValue.code);
      if (item) {
        if (!elementUsedMap[name]) {
          elementUsedMap[name] = [];
        }
        if (elementUsedMap[name].length < 2) {
          elementUsedMap[name].push({ code: item.code, value: buffValue.value });
          item.value += buffValue.value;
        }
      }
    }

    function isValid(buildItems: BuildItem[]) {
      return buildItems.every(item => item.value >= item.level * 5);
    }

    function tryCombination(index: number, buildItems: BuildItem[], elementUsedMap: Record<string, { code: string | number; value: number }[]>) {
      if (index === 5) {
        if (isValid(buildItems)) {
          // const mapKeySortBuildItems = buildItems.sort((a, b) => a.code - b.code);

          // const mapKey = mapKeySortBuildItems.reduce((acc, item) => {
          //   acc.push(`${item.code}-${item.value}`);
          //   return acc;
          // }, []).join(',');

          const filteredElementUsedArray = Object.keys(elementUsedMap).reduce((acc, key) => {
            if (usedAccessoryNameArray.includes(key)) {
              const filterKey = key.startsWith('Earring') ? 'Earring' : key.startsWith('Ring') ? 'Ring' : key;

              acc.push(`${filterKey}:${elementUsedMap[key].map(e => `${e.code}-${e.value}`).join(',')}`);
            }
            return acc;
          }, [] as string[]);

          // if (!resultArray.has(mapKey)) {
          //   resultArray.set(mapKey, new Set());
          // }

          // const resultArraySet = resultArray.get(mapKey);

          const sortPartFilteredElementUsedString = filteredElementUsedArray.map(part => {
            let [type, value] = part.split(":");
            let sortedValues = value.split(",").sort().join(",");
            return `${type}:${sortedValues}`;
          }).sort().join("|");

          if (!resultArray.has(sortPartFilteredElementUsedString)) {
            resultArray.add(sortPartFilteredElementUsedString);
            filteredElementUsedArray.forEach(e => totalUsedAccessorySet.add(e));
            return
          }
        }
        return
      }

      const points = engravingPointDistributions;

      for (let i = 0; i < points.length; i++) {
        const pointPair = points[i];

        for (let j = 0; j < buildItems.length; j++) {
          const item1 = buildItems[j];
          if (item1.value < item1.level * 5 && !elementUsedMap[usedAccessoryNameArray[index]]?.find(e => e.code === item1.code)) {
            item1.value += pointPair[0];
            if (!elementUsedMap[usedAccessoryNameArray[index]]) {
              elementUsedMap[usedAccessoryNameArray[index]] = [];
            }
            elementUsedMap[usedAccessoryNameArray[index]].push({ code: item1.code, value: pointPair[0] });

            for (let k = 0; k < buildItems.length; k++) {
              if (k !== j) {
                const item2 = buildItems[k];
                if (item2.value < item2.level * 5 && !elementUsedMap[usedAccessoryNameArray[index]]?.find(e => e.code === item2.code)) {
                  item2.value += pointPair[1];
                  elementUsedMap[usedAccessoryNameArray[index]].push({ code: item2.code, value: pointPair[1] });

                  tryCombination(index + 1, buildItems, elementUsedMap);

                  item2.value -= pointPair[1];
                  elementUsedMap[usedAccessoryNameArray[index]].pop();
                }
              }
            }

            item1.value -= pointPair[0];
            elementUsedMap[usedAccessoryNameArray[index]].pop();
          }
        }
      }
    }

    const elementUsedMap = {};
    addSettingValue('Stone_1', buildParam.stone_builds.buff_1, buildItems, elementUsedMap);
    addSettingValue('Stone_2', buildParam.stone_builds.buff_2, buildItems, elementUsedMap);
    addSettingValue('Self_1', buildParam.self_builds.buff_1, buildItems, elementUsedMap);
    addSettingValue('Self_2', buildParam.self_builds.buff_2, buildItems, elementUsedMap);

    tryCombination(0, buildItems, elementUsedMap);

    resolve({
      result_array: Array.from(resultArray),
      total_used_accessory_array: Array.from(totalUsedAccessorySet).sort()
    })
  })
}

function calculate_price(param: [Calculate.CalculatePriceParam[], string[]]): Promise<Calculate.CalculatePriceResult[]> {
  return new Promise((resolve) => {
    const [price_array, items_array] = param;

    const items_price_array = items_array.reduce((acc, cur) => {
      const items = cur.split("|");

      function parseItem(item: string) {
        const [accessory, buffs] = item.split(":");
        const [buff1, buff2] = buffs.split(",");
        const [buff1_code, buff1_value] = buff1.split("-");
        const [buff2_code, buff2_value] = buff2.split("-");
        return { accessory, buff1_code, buff1_value: Number(buff1_value), buff2_code, buff2_value: Number(buff2_value) };
      }

      function findBuild(parsedItem: ReturnType<typeof parseItem>) {
        return price_array.find(e =>
          e.accessory === parsedItem.accessory &&
          e.build[parsedItem.buff1_code] === parsedItem.buff1_value &&
          e.build[parsedItem.buff2_code] === parsedItem.buff2_value
        );
      }

      const valid_items = items.every(item => {
        const parsedItem = parseItem(item);
        const build = findBuild(parsedItem);
        return build && build.price !== 0;
      });

      if (valid_items) {
        const used_items = items.map(item => {
          const parsedItem = parseItem(item);
          const build = findBuild(parsedItem)!;
          return {
            accessory_name: parsedItem.accessory,
            accessory: build.accessory,
            build_string: build.build_string,
            build: build.build,
            is_artifact: build.is_artifact,
            is_artifact_disabled: build.is_artifact_disabled,
            price: build.price,
            base_string: build.base_string
          };
        });

        const price_total = used_items.reduce((acc, item) => acc + item.price, 0);

        acc.push({ used_items, price_total });
      }

      return acc;
    }, [] as Calculate.CalculatePriceResult[]);

    resolve(items_price_array.filter(item => item.price_total > 0).sort((a, b) => a.price_total - b.price_total));
  });
}

export {
  calculate_build,
  calculate_price
}
