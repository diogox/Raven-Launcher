mod options;
mod preference;

// TODO: Implement a method to validate this properly
#[derive(Deserialize, Debug)]
pub struct ExtensionManifest {

    ///  
    manifest_version: String,
    
    /// 
    api_version: String,
    
    /// 
    name: String,

    /// 
    #[serde(default)]
    description: String,

    /// 
    #[serde(default)]
    developer_name: String,

    /// 
    #[serde(default)]
    #[serde(rename = "icon")]
    icon_path: Option<String>,

    ///
    options: options::Options,

    ///
    preferences: Vec<preference::Preference>,
}

impl ExtensionManifest {

    pub fn validate(manifest: &Self) -> Result<(), ()> {

        
        // Validate Preferences
        let pref_error = manifest.preferences
            .iter()
            .find(|pref| preference::Preference::validate(pref).is_err());
        
        // Check for any errors
        if pref_error.is_some()  {
            panic!("Errors found while parsing the manifest's preferences!");
        }

        Ok( () )
    } 

    // Getters
    pub fn trigger_keyword(&self) -> String {

        // Find preference
        let preference = self.preferences.iter()
            .find(|p| p.is_keyword_preference())
            .expect("Manifest doesn't have default trigger keyword!");

        // Get default trigger
        let keyword = preference.default_value();
        
        // Return String version of it
        keyword.to_owned()
    }
}