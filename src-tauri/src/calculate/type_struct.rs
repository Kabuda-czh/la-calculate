use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BuildItem {
    pub code: String,
    pub level: u32,
    pub value: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsedElement {
    pub code: String,
    pub value: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Build {
    pub code: String,
    pub level: u32,
    pub need_value: Option<u32>,
    pub value: Option<u32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoneBuild {
    pub buff_1: Buff,
    pub buff_2: Buff,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SelfBuild {
    pub buff_1: Buff,
    pub buff_2: Buff,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Buff {
    pub code: String,
    pub value: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CalculatePageParam {
    pub need_builds: Vec<Build>,
    pub stone_builds: StoneBuild,
    pub self_builds: SelfBuild,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CalculateResult {
    pub result_array: Vec<String>,
    pub total_used_accessory_array: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CalculatePriceParam {
    pub accessory_name: String,
    pub accessory: String,
    pub build_string: String,
    pub build: HashMap<String, u32>,
    pub is_artifact: u32,
    pub is_artifact_disabled: bool,
    pub price: u32,
    pub is_buy: bool,
    pub base_string: String,
    pub remark: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CalculatePriceResult {
    pub used_items: Vec<CalculatePriceParam>,
    pub price_total: u32,
}
