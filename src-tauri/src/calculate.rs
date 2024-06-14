pub mod type_struct;

use std::collections::{HashMap, HashSet};

#[warn(unused_imports)]
use type_struct::{
    Buff, BuildItem, CalculatePageParam, CalculatePriceParam, CalculatePriceResult,
    CalculateResult, UsedElement,
};

#[warn(dead_code)]
const USED_ACCESSORY_NAME_ARRAY: [&str; 5] =
    ["Amulet", "Earring_1", "Earring_2", "Ring_1", "Ring_2"];

pub async fn calculate_build_fn(
    build_param: CalculatePageParam,
    artifact_check: bool,
) -> CalculateResult {
    let mut engraving_point_distributions = vec![(4, 3), (5, 3), (6, 3)];

    if artifact_check {
        engraving_point_distributions.pop();
    }

    let mut result_array: HashSet<String> = HashSet::new();
    let mut total_used_accessory_set: HashSet<String> = HashSet::new();

    let mut build_items = build_param
        .need_builds
        .iter()
        .map(|build| BuildItem {
            code: build.code.to_string(),
            level: build.level,
            value: 0,
        })
        .collect::<Vec<_>>();

    fn add_setting_value<'a>(
        name: &'a str,
        buff_value: &Buff,
        build_items: &mut Vec<BuildItem>,
        element_used_map: &mut HashMap<&'a str, Vec<UsedElement>>,
    ) {
        if let Some(item) = build_items
            .iter_mut()
            .find(|element| element.code == buff_value.code)
        {
            let element_used = element_used_map.entry(name).or_insert(Vec::new());

            if element_used.len() < 2 {
                element_used.push(UsedElement {
                    code: item.code.to_string(),
                    value: buff_value.value,
                });
                item.value += buff_value.value;
            }
        }
    }

    fn is_valid(build_items: &mut Vec<BuildItem>) -> bool {
        build_items.iter().all(|item| item.value >= item.level * 5)
    }

    fn try_combination(
        index: usize,
        build_items: &mut Vec<BuildItem>,
        element_used_map: &mut HashMap<&str, Vec<UsedElement>>,
        result_array: &mut HashSet<String>,
        total_used_accessory_set: &mut HashSet<String>,
        engraving_point_distributions: &[(u32, u32)],
    ) {
        if index == 5 {
            if is_valid(build_items) {
                let filtered_element_used_array = element_used_map
                    .iter()
                    .filter(|(key, _)| USED_ACCESSORY_NAME_ARRAY.contains(key))
                    .map(|(key, value)| {
                        let filter_key = if key.starts_with("Earring") {
                            "Earring"
                        } else if key.starts_with("Ring") {
                            "Ring"
                        } else {
                            key
                        };
                        let value_array = value
                            .iter()
                            .map(|e| format!("{}-{}", e.code, e.value))
                            .collect::<Vec<_>>();

                        if value_array.len() < 2 {
                            println!("{} {:?} {:?}", key, value, element_used_map.get(key));
                        }

                        format!(
                            "{}:{}",
                            filter_key,
                            value
                                .iter()
                                .map(|e| format!("{}-{}", e.code, e.value))
                                .collect::<Vec<_>>()
                                .join(",")
                        )
                    })
                    .collect::<Vec<_>>();

                let mut sort_part_filtered_element_used_string_array = filtered_element_used_array
                    .iter()
                    .map(|part| {
                        let mut split = part.split(":");
                        let type_string = split.next().unwrap();
                        let mut values = split.next().unwrap().split(",").collect::<Vec<_>>();
                        values.sort();
                        format!("{}:{}", type_string, values.join(","))
                    })
                    .collect::<Vec<_>>();

                sort_part_filtered_element_used_string_array.sort();

                let sort_part_filtered_element_used_string =
                    sort_part_filtered_element_used_string_array.join("|");

                if !result_array.contains(&sort_part_filtered_element_used_string) {
                    result_array.insert(sort_part_filtered_element_used_string);
                    for e in filtered_element_used_array {
                        total_used_accessory_set.insert(e);
                    }
                    return;
                }
            }
            return;
        }

        let build_items_length = build_items.len();

        for point_pair in engraving_point_distributions {
            for i in 0..build_items_length {
                let find_element = element_used_map
                    .get(&USED_ACCESSORY_NAME_ARRAY[index])
                    .and_then(|e| e.iter().find(|e| e.code == build_items[i].code));

                if build_items[i].value < build_items[i].level * 5 && find_element.is_none() {
                    build_items[i].value += point_pair.0;
                    element_used_map
                        .entry(&USED_ACCESSORY_NAME_ARRAY[index])
                        .or_insert_with(|| Vec::new())
                        .push(UsedElement {
                            code: build_items[i].code.to_string(),
                            value: point_pair.0,
                        });

                    for j in 0..build_items_length {
                        if j != i {
                            let find_element = element_used_map
                                .get(&USED_ACCESSORY_NAME_ARRAY[index])
                                .and_then(|e| e.iter().find(|e| e.code == build_items[j].code));

                            if build_items[j].value < build_items[j].level * 5
                                && find_element.is_none()
                            {
                                build_items[j].value += point_pair.1;
                                element_used_map
                                    .entry(&USED_ACCESSORY_NAME_ARRAY[index])
                                    .or_insert_with(|| Vec::new())
                                    .push(UsedElement {
                                        code: build_items[j].code.to_string(),
                                        value: point_pair.1,
                                    });

                                try_combination(
                                    index + 1,
                                    build_items,
                                    element_used_map,
                                    result_array,
                                    total_used_accessory_set,
                                    engraving_point_distributions,
                                );

                                build_items[j].value -= point_pair.1;
                                element_used_map
                                    .get_mut(&USED_ACCESSORY_NAME_ARRAY[index])
                                    .unwrap()
                                    .pop();
                            }
                        }
                    }

                    build_items[i].value -= point_pair.0;
                    element_used_map
                        .get_mut(&USED_ACCESSORY_NAME_ARRAY[index])
                        .unwrap()
                        .pop();
                }
            }
        }
    }

    let mut element_used_map: HashMap<&str, Vec<UsedElement>> = HashMap::new();

    add_setting_value(
        "Stone_1",
        &build_param.stone_builds.buff_1,
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Stone_2",
        &build_param.stone_builds.buff_2,
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Self_1",
        &build_param.self_builds.buff_1,
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Self_2",
        &build_param.self_builds.buff_2,
        &mut build_items,
        &mut element_used_map,
    );

    try_combination(
        0,
        &mut build_items,
        &mut element_used_map,
        &mut result_array,
        &mut total_used_accessory_set,
        &engraving_point_distributions,
    );

    CalculateResult {
        result_array: result_array.into_iter().collect(),
        total_used_accessory_array: total_used_accessory_set.into_iter().collect(),
    }
}

#[derive(Debug)]
struct ParseItem {
    accessory: String,
    buff1_code: String,
    buff1_value: u32,
    buff2_code: String,
    buff2_value: u32,
}

pub async fn calculate_price_fn<'a>(
    param: (Vec<CalculatePriceParam>, Vec<&str>),
) -> Vec<CalculatePriceResult> {
    let (price_array, items_array) = param;

    let mut amulet_array: Vec<CalculatePriceParam> = price_array
        .iter()
        .cloned()
        .filter(|e| e.accessory == "Amulet")
        .collect();
    let mut earring_array: Vec<CalculatePriceParam> = price_array
        .iter()
        .cloned()
        .filter(|e| e.accessory == "Earring")
        .collect();
    let mut ring_array: Vec<CalculatePriceParam> = price_array
        .iter()
        .cloned()
        .filter(|e| e.accessory == "Ring")
        .collect();

    let mut earring_buy_one: Vec<CalculatePriceParam> = vec![];
    let mut ring_buy_one: Vec<CalculatePriceParam> = vec![];

    let is_buy_amulet_count = amulet_array.iter().filter(|e| e.is_buy).count();
    let is_buy_earring_count = earring_array.iter().filter(|e| e.is_buy).count();
    let is_buy_ring_count = ring_array.iter().filter(|e| e.is_buy).count();

    if is_buy_amulet_count == 1 {
        amulet_array = amulet_array.into_iter().filter(|e| e.is_buy).collect();
    }
    if is_buy_earring_count == 1 {
        earring_buy_one.push(
            earring_array
                .iter()
                .cloned()
                .filter(|e| e.is_buy)
                .next()
                .unwrap(),
        );
    } else if is_buy_earring_count == 2 {
        earring_array = earring_array.into_iter().filter(|e| e.is_buy).collect();
    }
    if is_buy_ring_count == 1 {
        ring_buy_one.push(
            ring_array
                .iter()
                .cloned()
                .filter(|e| e.is_buy)
                .next()
                .unwrap(),
        );
    } else if is_buy_ring_count == 2 {
        ring_array = ring_array.into_iter().filter(|e| e.is_buy).collect();
    }

    let price_array_filtered: Vec<CalculatePriceParam> = amulet_array
        .into_iter()
        .chain(earring_array.into_iter())
        .chain(ring_array.into_iter())
        .filter(|e| e.price > 0)
        .collect();

    fn is_invalid_accessory(
      items: &Vec<&str>,
      accessory_array: &Vec<CalculatePriceParam>
    ) -> bool {
        if accessory_array.len() == 1 {
            let base_string = accessory_array[0].base_string.as_str();
            return !items.contains(&base_string) || items.iter().filter(|&item| item == &base_string).count() > 1;
        }
        return false;
    }

    let items_price_array = items_array.iter().fold(vec![], |mut acc, cur| {
        let items: Vec<&str> = cur.split('|').collect();

        if is_invalid_accessory(&items, &earring_buy_one) || is_invalid_accessory(&items, &ring_buy_one) {
            return acc;
        }

        fn parse_item(item: &str) -> ParseItem {
            let parts: Vec<&str> = item.split(':').collect();
            let accessory = parts[0].to_string();
            let buffs: Vec<&str> = parts[1].split(',').collect();
            let buff1: Vec<&str> = buffs[0].split('-').collect();
            let buff2: Vec<&str> = buffs[1].split('-').collect();
            ParseItem {
                accessory,
                buff1_code: buff1[0].to_string(),
                buff1_value: buff1[1].parse().unwrap(),
                buff2_code: buff2[0].to_string(),
                buff2_value: buff2[1].parse().unwrap(),
            }
        }

        fn find_build(
            parsed_item: &ParseItem,
            price_array_filtered: &Vec<CalculatePriceParam>,
        ) -> Option<CalculatePriceParam> {
            price_array_filtered.iter().cloned().find(|e| {
                e.accessory == parsed_item.accessory
                    && e.build.get(&parsed_item.buff1_code) == Some(&parsed_item.buff1_value)
                    && e.build.get(&parsed_item.buff2_code) == Some(&parsed_item.buff2_value)
            })
        }

        let valid_items = items.iter().all(|&item| {
            let parsed_item = parse_item(item);
            let build = find_build(&parsed_item, &price_array_filtered);
            build.is_some() && build.unwrap().price != 0
        });

        if valid_items {
            let used_items: Vec<CalculatePriceParam> = items
                .iter()
                .map(|&item| {
                    let parsed_item = parse_item(item);
                    find_build(&parsed_item, &price_array_filtered).unwrap()
                })
                .collect();

            let price_total = used_items.iter().map(|item| item.price).sum();

            acc.push(CalculatePriceResult {
                used_items,
                price_total,
            });
        }

        acc
    });

    let mut items_price_filtered_array = items_price_array
    .into_iter()
    .filter(|item| item.price_total > 0)
    .collect::<Vec<_>>();

    items_price_filtered_array.sort_by(|a, b| a.price_total.cmp(&b.price_total));

    return items_price_filtered_array;
}
