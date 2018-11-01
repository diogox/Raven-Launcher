use std::{
    io,
    env,
    path::{
        Path,
        PathBuf,
    },
};

pub fn ls() -> Result< Option< Vec<String> > , std::io::Error> {
    let mut all_files = Vec::new();
    for appsdir in find_application_dirs()? {
        let desktop_files = ls_one_dir(&appsdir)?;
        if let Some(mut files) = desktop_files {
            all_files.append(files.as_mut());
        }
    }

    if all_files.is_empty() {
        return Ok( None );
    }

    Ok( Some(all_files) )
}

fn find_application_dirs() -> io::Result<Vec<PathBuf>> {
    let data_home = match env::var_os("XDG_DATA_HOME") {
        Some(val) => {
            PathBuf::from(val)
        },
        None => {
            let home = env::home_dir().ok_or(io::Error::new(io::ErrorKind::Other, "Couldn't get home dir"))?;
            home.join(".local/share")
        }
    };
    let extra_data_dirs = match env::var_os("XDG_DATA_DIRS") {
        Some(val) => {
            env::split_paths(&val).map(PathBuf::from).collect()
        },
        None => {
            vec![PathBuf::from("/usr/local/share"),
                 PathBuf::from("/usr/share")]
        }
    };

    let mut res = Vec::new();
    res.push(data_home.join("applications"));
    for dir in extra_data_dirs {
        res.push(dir.join("applications"));
    }
    Ok(res)
}

fn ls_one_dir(path : &Path) -> Result<Option<Vec<String>>, std::io::Error> {
    
    if !path.is_dir() {
        return Ok( None )
    }

    let mut filenames = get_dir_desktop_files(&path)?.iter()
                        .map(|e| e.file_name().to_string_lossy().into_owned())
                        .collect::<Vec<_>>();

    if filenames.is_empty() {
        return Ok( None )
    }

    filenames.sort_by_key(|f| f.to_lowercase());

    let mut files = Vec::new();
    for filename in filenames {
        files.push(filename);
    }

    Ok(Some(files))
}

fn get_dir_desktop_files(path: &Path) -> io::Result<Vec<std::fs::DirEntry>> {
    return Ok(path.read_dir()?
             .filter_map(|v| v.ok())
             .filter(|e| match e.file_type() {
                 Ok(ft) => (ft.is_file() | ft.is_symlink()),
                 _ => false
              })
             .filter(|e| e.file_name().to_string_lossy().ends_with(".desktop"))
             .collect::<Vec<_>>());
}