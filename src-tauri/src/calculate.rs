use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Build {
    code: i32,
    level: i32,
    value: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Buff {
    code: i32,
    value: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildParam {
    need_builds: Vec<Build>,
    stone_builds: HashMap<String, Buff>,
    self_builds: HashMap<String, Buff>,
}

fn add_setting_value(
    name: &str,
    buff_value: &Buff,
    build_items: &mut Vec<Build>,
    element_used_map: &mut HashMap<String, Vec<Buff>>,
) {
    if let Some(item) = build_items
        .iter_mut()
        .find(|element| element.code == buff_value.code)
    {
        element_used_map
            .entry(name.to_string())
            .or_insert(Vec::new())
            .push(buff_value.clone());
        item.value += buff_value.value;
    }
}

fn is_valid(build_items: &[Build]) -> bool {
    build_items.iter().all(|item| item.value >= item.level * 5)
}

fn try_combination(
    index: usize,
    build_items: &mut Vec<Build>,
    element_used_map: &mut HashMap<String, Vec<Buff>>,
    result_array: &mut HashSet<String>,
    total_used_ass_set: &mut HashSet<String>,
    used_accessory_name_array: &[&str],
    engraving_point_distributions: &[[i32; 2]],
) {
    if index == 5 {
        if is_valid(build_items) {
            let filtered_element_used_array: Vec<String> = element_used_map
                .iter()
                .filter(|(key, _)| used_accessory_name_array.contains(&key.as_str()))
                .map(|(key, value)| {
                    let filter_key = if key.starts_with("Earring") {
                        "Earring"
                    } else if key.starts_with("Ring") {
                        "Ring"
                    } else {
                        key
                    };

                    format!(
                        "{}:{}",
                        filter_key,
                        value
                            .iter()
                            .map(|e| format!("{}-{}", e.code, e.value))
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                })
                .collect();

            let sort_part_filtered_element_used_string = filtered_element_used_array
                .iter()
                .map(|part| {
                    let parts: Vec<&str> = part.split(':').collect();
                    let sorted_values = parts[1].split(',').collect::<Vec<&str>>().join(",");
                    format!("{}:{}", parts[0], sorted_values)
                })
                .collect::<Vec<String>>()
                .join("|");

            if !result_array.contains(&sort_part_filtered_element_used_string) {
                result_array.insert(sort_part_filtered_element_used_string);
                for e in filtered_element_used_array {
                    total_used_ass_set.insert(e);
                }
            }
        }
        return;
    }

    for point_pair in engraving_point_distributions {
        for j in 0..build_items.len() {
            if build_items[j].value < build_items[j].level * 5
                && !element_used_map
                    .get(used_accessory_name_array[index])
                    .map_or(false, |v| v.iter().any(|e| e.code == build_items[j].code))
            {
                build_items[j].value += point_pair[0];
                element_used_map
                    .entry(used_accessory_name_array[index].to_string())
                    .or_insert(Vec::new())
                    .push(Buff {
                        code: build_items[j].code,
                        value: point_pair[0],
                    });

                for k in 0..build_items.len() {
                    if k != j {
                        if build_items[k].value < build_items[k].level * 5
                            && !element_used_map
                                .get(used_accessory_name_array[index])
                                .map_or(false, |v| v.iter().any(|e| e.code == build_items[k].code))
                        {
                            build_items[k].value += point_pair[1];
                            element_used_map
                                .get_mut(used_accessory_name_array[index])
                                .unwrap()
                                .push(Buff {
                                    code: build_items[k].code,
                                    value: point_pair[1],
                                });

                            try_combination(
                                index + 1,
                                build_items,
                                element_used_map,
                                result_array,
                                total_used_ass_set,
                                used_accessory_name_array,
                                engraving_point_distributions,
                            );

                            build_items[k].value -= point_pair[1];
                            element_used_map
                                .get_mut(used_accessory_name_array[index])
                                .unwrap()
                                .pop();
                        }
                    }
                }

                build_items[j].value -= point_pair[0];
                element_used_map
                    .get_mut(used_accessory_name_array[index])
                    .unwrap()
                    .pop();
            }
        }
    }
}

pub fn calculate(build_param: BuildParam) -> (Vec<String>, Vec<String>) {
    let mut result_array = HashSet::new();
    let mut total_used_ass_set = HashSet::new();
    let mut build_items: Vec<Build> = build_param
        .need_builds
        .iter()
        .map(|build| Build {
            code: build.code,
            level: build.level,
            value: 0,
        })
        .collect();
    let mut element_used_map: HashMap<String, Vec<Buff>> = HashMap::new();
    let used_accessory_name_array = ["Amulet", "Earring_1", "Earring_2", "Ring_1", "Ring_2"];
    let engraving_point_distributions = [
        // [4, 3],
        [5, 3],
        [6, 3],
    ];

    add_setting_value(
        "Stone_1",
        &build_param.stone_builds["buff_1"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Stone_2",
        &build_param.stone_builds["buff_2"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Self_1",
        &build_param.self_builds["buff_1"],
        &mut build_items,
        &mut element_used_map,
    );
    add_setting_value(
        "Self_2",
        &build_param.self_builds["buff_2"],
        &mut build_items,
        &mut element_used_map,
    );

    try_combination(
        0,
        &mut build_items,
        &mut element_used_map,
        &mut result_array,
        &mut total_used_ass_set,
        &used_accessory_name_array,
        &engraving_point_distributions,
    );

    (
        result_array.into_iter().collect(),
        total_used_ass_set.into_iter().collect(),
    )
}
