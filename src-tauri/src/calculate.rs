use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub code: i32,
    pub level: i32,
    pub need_value: Option<i32>,
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buff {
    pub code: i32,
    pub value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuildParam {
    pub need_builds: Vec<Build>,
    pub stone_builds: HashMap<String, Buff>,
    pub self_builds: HashMap<String, Buff>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculateResult {
    pub result_array: HashMap<String, HashSet<Vec<String>>>,
    pub total_used_ass_set: HashSet<String>,
}

fn add_setting_value(
    name: &str,
    buff: &Buff,
    build_items: &mut Vec<Build>,
    element_used_map: &mut HashMap<String, Vec<Buff>>,
) {
    if let Some(item) = build_items.iter_mut().find(|item| item.code == buff.code) {
        element_used_map
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(buff.clone());
        item.value = Some(item.value.unwrap_or(0) + buff.value);
    }
}

fn is_valid(build_items: &Vec<Build>) -> bool {
    build_items.iter().all(|item| item.value.unwrap_or(0) >= item.need_value.unwrap_or(0))
}

fn try_combination(
    index: usize,
    build_items: &mut Vec<Build>,
    element_used_map: &mut HashMap<String, Vec<Buff>>,
    ass_array: &Vec<Vec<i32>>,
    used_ass_name_array: &Vec<&str>,
    result_array: &mut HashMap<String, HashSet<Vec<String>>>,
    total_used_ass_set: &mut HashSet<String>,
) -> bool {
    if index == 5 {
        if is_valid(build_items) {
            let mut sorted_build_items = build_items.clone();
            sorted_build_items.sort_by_key(|item| item.code);
            let key = serde_json::to_string(&sorted_build_items).unwrap();

            let filtered_element_used_array: Vec<String> = element_used_map
                .iter()
                .filter(|&(k, _)| used_ass_name_array.contains(&k.as_str()))
                .map(|(k, v)| {
                    let mut buffs: Vec<String> = v
                        .iter()
                        .map(|e| format!("{}-{}", e.code, e.value))
                        .collect();
                    buffs.sort();
                    format!("{}:{}", k, buffs.join(","))
                })
                .collect();

            result_array
                .entry(key)
                .or_insert_with(HashSet::new)
                .insert(filtered_element_used_array.clone());
            for e in &filtered_element_used_array {
                total_used_ass_set.insert(e.clone());
            }
            return true;
        }
        return false;
    }

    let values = ass_array;

    for value_pair in values {
        for j in 0..build_items.len() {
            {
                let item1 = &mut build_items[j];
                if item1.value.unwrap_or(0) < item1.need_value.unwrap_or(0)
                    && element_used_map
                        .get(used_ass_name_array[index])
                        .map_or(true, |v| !v.iter().any(|e| e.code == item1.code))
                {
                    item1.value = Some(item1.value.unwrap_or(0) + value_pair[0]);
                    element_used_map
                        .entry(used_ass_name_array[index].to_string())
                        .or_insert_with(Vec::new)
                        .push(Buff {
                            code: item1.code,
                            value: value_pair[0],
                        });
                } else {
                    continue;
                }
            }

            for k in 0..build_items.len() {
                if k == j {
                    continue;
                }

                let mut item2_code = 0;
                let mut item2_value = 0;
                let mut item2_need_value = 0;

                {
                    let item2 = &mut build_items[k];
                    if item2.value.unwrap_or(0) < item2.need_value.unwrap_or(0)
                        && element_used_map
                            .get(used_ass_name_array[index])
                            .map_or(true, |v| !v.iter().any(|e| e.code == item2.code))
                    {
                        item2.value = Some(item2.value.unwrap_or(0) + value_pair[1]);
                        element_used_map
                            .get_mut(used_ass_name_array[index])
                            .unwrap()
                            .push(Buff {
                                code: item2.code,
                                value: value_pair[1],
                            });

                        item2_code = item2.code;
                        item2_value = item2.value.unwrap_or(0);
                        item2_need_value = item2.need_value.unwrap_or(0);
                    } else {
                        continue;
                    }
                }

                if try_combination(
                    index + 1,
                    build_items,
                    element_used_map,
                    values,
                    used_ass_name_array,
                    result_array,
                    total_used_ass_set,
                ) {
                    return true;
                }

                {
                    let item2 = &mut build_items[k];
                    item2.value = Some(item2.value.unwrap_or(0) - value_pair[1]);
                    element_used_map
                        .get_mut(used_ass_name_array[index])
                        .unwrap()
                        .pop();
                }
            }

            let item1 = &mut build_items[j];
            item1.value = Some(item1.value.unwrap_or(0) - value_pair[0]);
            element_used_map
                .get_mut(used_ass_name_array[index])
                .unwrap()
                .pop();
        }
    }
    false
}

pub fn calculate(build_param: BuildParam) -> CalculateResult {
    let ass_array = vec![vec![5, 3], vec![6, 3]];
    let used_ass_name_array = vec!["A", "B", "C", "D", "E"];

    let mut result_array: HashMap<String, HashSet<Vec<String>>> = HashMap::new();
    let mut total_used_ass_set: HashSet<String> = HashSet::new();
    let mut build_items = build_param.need_builds.iter().map(|build| Build {
        code: build.code,
        level: build.level,
        need_value: Some(build.level * 5),
        value: Some(0),
    }).collect::<Vec<Build>>();

    let mut element_used_map: HashMap<String, Vec<Buff>> = HashMap::new();

    add_setting_value(
        "stone_buff_1",
        &build_param.stone_builds["buff_1"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "stone_buff_2",
        &build_param.stone_builds["buff_2"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "self_buff_1",
        &build_param.self_builds["buff_1"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "self_buff_2",
        &build_param.self_builds["buff_2"],
        &mut build_items,
        &mut element_used_map,
    );

    try_combination(
        0,
        &mut build_items,
        &mut element_used_map,
        &ass_array,
        &used_ass_name_array,
        &mut result_array,
        &mut total_used_ass_set,
    );

    CalculateResult {
        result_array,
        total_used_ass_set,
    }
}
