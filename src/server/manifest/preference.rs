#[derive(Deserialize, Debug)]
pub struct Preference {

    /// Key that is used to 
    /// retrieve value for a 
    /// certain preference.
    id: String,

    ///
    #[serde(rename = "type")]
    preference_type: Type,

    /// 
    name: String,

    /// 
    default_value: String,
    
    /// 
    #[serde(default)]
    description: String,

    /// 
    #[serde(default)]
    options: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
enum Type {

    #[serde(rename = "keyword")]
    Keyword,
    
    #[serde(rename = "input")]
    Input,

    #[serde(rename = "text")]
    TextArea,

    #[serde(rename = "select")]
    Select,
}