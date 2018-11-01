use std::path::PathBuf;
use rusqlite::Connection;
use gio::{
    AppInfo,
    AppInfoExt,
};

use ::api::items::ExtensionResultItem;
use super::app::App;

pub struct AppDb {
    db: Connection,
    // TODO: Save icons in a hashmap on 'put_app'.
}

impl AppDb {

    pub fn new() -> Self {

        // Start db
        let db = Connection::open_in_memory()
            .unwrap();

        // Create table
        db.execute(
        "CREATE TABLE app_db (
            name VARCHAR,
            desktop_file VARCHAR,
            desktop_file_short VARCHAR PRIMARY KEY,
            description VARCHAR,
            search_name VARCHAR
        );
        CREATE INDEX desktop_file_idx ON app_db (desktop_file);
        ", &[]).unwrap();

        let app_db = AppDb {
            db,
        };

        // Get all registred apps
        let apps_info = AppInfo::get_all();

        // Populate the database
        for app_info in apps_info {

            // Only apps that should be listed
            // http://gtk-rs.org/docs/gio/trait.AppInfoExt.html
            if app_info.should_show() {
                let app = App::builder()
                    .name(app_info.get_display_name().unwrap())
                    .desktop_file( // ! I put the executable here instead of the desktop file
                        app_info.get_executable()
                            .unwrap()
                    )
                    .description(
                        app_info.get_description()
                            .unwrap_or("".to_string())
                    )
                    .search_name("".to_string())
                    .build();
                app_db.put_app(app)
            }
        }

        app_db
    }

    pub fn put_app(&self, app: App) {

        // Get the desktop file's name
        let desktop_file_short = app.desktop_file()
            .file_name()
            .unwrap()
            .to_str();

        // Add to db
        self.db.execute(
        "INSERT OR REPLACE INTO app_db (name, desktop_file, desktop_file_short, description, search_name)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        &[
            &app.name(),
            &app.desktop_file().to_str(),
            &desktop_file_short,
            &app.description(),
            &app.search_name(),
        ]
        ).expect("Failed to insert an app in the database!");
    }

    pub fn by_name(&self, name: &str) -> Option<App> {
        let query = format!("SELECT * FROM app_db where name = {} COLLATE NOCASE", name);
        let error_msg = format!("Couldn't find '{}' in the apps database", name);
        
        // Get app with that name from db
        let record = self.db.query_row(
            &query,
            &[],
            |row| {
                let desktop_file: Result<String, _> = row.get_checked(1);
                match desktop_file {
                    Ok(df) => {

                        // Get PathBuf
                        let mut desktop_file = PathBuf::new();
                        desktop_file.push(df);

                        // Build App
                        let app = App::builder()
                            .name(row.get::<_, String>(0))
                            .desktop_file(desktop_file)
                            .description(row.get::<_, String>(2))
                            .search_name(row.get::<_, String>(3))
                            .build();

                        Some(app)
                    }
                    Err(_) => { return None }
                }
            }
        ).expect(&error_msg);

        record
    }

    pub fn search(&self, query: &str) -> Vec< ExtensionResultItem > {
        if query.len() == 0 {
            return self.get_all();
        }

        // Search for query
        let mut results_query = self.db.prepare(&format!("SELECT * FROM app_db WHERE name LIKE '%{}%'", query))
            .unwrap();
        
        let results = results_query.query_map(&[], 
            |row| {
                let mut result = ExtensionResultItem::new();
                result.set_name(row.get::<_, String>(0));
                result.set_description(row.get::<_, String>(4));
                // TODO: Implement this when there's a struct member that svaes these values
                //.icon()
                // ! Field doesn't exist
                //.search_name(row.get(5))
                result
            }
        ).expect("Failed to map app results!");

        let mut items = Vec::new();
        for result in results {
            items.push(result.unwrap());
        }

        items
    }

    fn get_all(&self) -> Vec< ExtensionResultItem > {

        let mut results_query = self.db.prepare("SELECT * FROM app_db")
            .unwrap();
        
        let results = results_query.query_map(&[], 
            |row| {
                let mut result = ExtensionResultItem::new();
                result.set_name(row.get::<_, String>(0));
                result.set_description(row.get::<_, String>(4));
                // TODO: Implement this when there's a struct member that svaes these values
                //.icon()
                // ! Field doesn't exist
                //.search_name(row.get(5))
                result
            }
        ).expect("Failed to map app results!");

        let mut items = Vec::new();
        for result in results {
            items.push(result.unwrap());
        }

        items
    }
}