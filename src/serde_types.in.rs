#[derive(Deserialize, Debug)]
pub struct ExchangeRate {
    pub disclaimer: String,
    pub license: String,
    pub timestamp: i64,
    pub base: String,
    pub rates: BTreeMap<String, f32>,
}

pub type Currencies = BTreeMap<String, String>;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Usage {
    pub status: u32,
    pub data: UsageData,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UsageData {
    pub app_id: String,
    pub status: String,
    pub plan: UsageDataPlan,
    pub usage: UsageDataUsage,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UsageDataPlan {
    pub name: String,
    pub quota: String,
    pub update_frequency: String,
    pub features: Features,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Features {
    pub base: bool,
    pub symbols: bool,
    pub experimental: bool,
    #[serde(rename = "time-series")]
    pub time_series: bool,
    pub convert: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct UsageDataUsage {
    pub requests: i64,
    pub requests_quota: i64,
    pub requests_remaining: i64,
    pub days_elapsed: i64,
    pub days_remaining: i64,
    pub daily_average: i64,
}