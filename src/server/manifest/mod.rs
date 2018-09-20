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
    icon: Option<String>,

    ///
    options: options::Options,

    ///
    preferences: Vec<preference::Preference>,
}