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

impl Preference {

    pub fn validate(preference: &Self) -> Result<(), ()> {
        let has_options_field = preference.options.is_some();
        
        // Error messages
        let missing_field = |f: &str| {
            let error_message = format!("The required field '{}' is missing from one of the manifest's preferences!", f);
            panic!(error_message);
        };

        let unused_field = |f: &str| {
            let error_message = format!("The field '{}' is not required in one of the manifest's preferences!", f);
            panic!(error_message);
        };

        // Check fields
        match preference.preference_type {
            Type::Input => {

                // Check for unnecessary 'options' field
                if has_options_field { unused_field("options") }
            },
            Type::Keyword => {

                // Check for unnecessary 'options' field
                if has_options_field { unused_field("options") }
            },
            Type::Select => {

                // Check for required 'options' field
                if !has_options_field { missing_field("options") }
            },
            Type::TextArea => {

                // Check for unnecessary 'options' field
                if has_options_field { unused_field("options") }
            },
        }
        Ok( () )
    }

    // Getters
    pub fn default_value(&self) -> &str { &self.default_value }

    pub fn is_keyword_preference(&self) -> bool {
        if let Type::Keyword = self.preference_type {
            return true;
        }
        false
    }
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