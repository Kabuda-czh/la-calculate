// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug)]
struct Element {
  code: String,
  value: i32
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // Test code
    use std::collections::HashMap;
    let ass_array = vec![
        vec![2, 2],
        vec![3, 2],
        vec![3, 3],
        vec![4, 3],
        vec![5, 3],
        vec![6, 3],
    ];

    let f = vec![9, 3];
    let g = 12;
    let h = 12;

    let required_3 = 5;
    let required_1 = 1;

    for a in 0..ass_array.len() {
        for b in 0..ass_array.len() {
            for c in 0..ass_array.len() {
                for d in 0..ass_array.len() {
                    for e in 0..ass_array.len() {
                        let mut t_array: Vec<Element> = Vec::new();
                        for i in 0..(required_1 + required_3) {
                            t_array.push(Element {
                                code: format!("T{}", i + 1),
                                value: 0,
                            });
                        }

                        let mut element_used_map: HashMap<String, Vec<Element>> = HashMap::new();
                        element_used_map.insert("A".to_string(), Vec::new());

                        fn add_value(
                            name: &str,
                            value: i32,
                            t_array: &mut Vec<Element>,
                            element_used_map: &mut HashMap<String, Vec<Element>>,
                        ) {
                            for t_element in t_array.iter_mut() {
                                if t_element.value < 15 && t_element.value + value <= 17 {
                                    let used_elements = element_used_map.entry(name.to_string()).or_insert(Vec::new());
                                    if used_elements.len() == 1 && used_elements[0].code != t_element.code {
                                        used_elements.push(Element {
                                            code: t_element.code.clone(),
                                            value,
                                        });
                                        t_element.value += value;
                                        break;
                                    } else if used_elements.is_empty() {
                                        used_elements.push(Element {
                                            code: t_element.code.clone(),
                                            value,
                                        });
                                        t_element.value += value;
                                        break;
                                    }
                                }
                            }
                        }

                        add_value("F", f[0], &mut t_array, &mut element_used_map);
                        add_value("F", f[1], &mut t_array, &mut element_used_map);
                        add_value("G", g, &mut t_array, &mut element_used_map);
                        add_value("H", h, &mut t_array, &mut element_used_map);

                        add_value("A", ass_array[a][0], &mut t_array, &mut element_used_map);
                        add_value("A", ass_array[a][1], &mut t_array, &mut element_used_map);

                        add_value("B", ass_array[b][0], &mut t_array, &mut element_used_map);
                        add_value("B", ass_array[b][1], &mut t_array, &mut element_used_map);

                        add_value("C", ass_array[c][0], &mut t_array, &mut element_used_map);
                        add_value("C", ass_array[c][1], &mut t_array, &mut element_used_map);

                        add_value("D", ass_array[d][0], &mut t_array, &mut element_used_map);
                        add_value("D", ass_array[d][1], &mut t_array, &mut element_used_map);

                        add_value("E", ass_array[e][0], &mut t_array, &mut element_used_map);
                        add_value("E", ass_array[e][1], &mut t_array, &mut element_used_map);

                        let count_3 = t_array.iter().filter(|el| el.value >= 15).count();
                        let count_1 = t_array.iter().filter(|el| el.value >= 5 && el.value < 15).count();

                        if count_3 == required_3 && count_1 == required_1 {
                            println!("found {:?} {:?}", t_array, element_used_map);
                        }
                    }
                }
            }
        }
    }
}
