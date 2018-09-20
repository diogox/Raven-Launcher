#[derive(Deserialize, Debug)]
pub struct Options {

    /// 
    #[serde(default = "default_query_debounce")]
    #[serde(rename = "query_debounce")]
    query_debounce_in_seconds: f64, // ? Should be duration
}

fn default_query_debounce() -> f64 {
    0.05
}