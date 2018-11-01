use std::path::PathBuf;

#[derive(TypedBuilder)]
pub struct App {
    name: String,
    desktop_file: PathBuf,
    description: String,
    search_name: String,
}

impl App {
    
    // Getters
    pub fn name(&self) -> &str { &self.name }
    pub fn desktop_file(&self) -> &PathBuf { &self.desktop_file }
    pub fn description(&self) -> &str { &self.description }
    pub fn search_name(&self) -> &str { &self.search_name }
}