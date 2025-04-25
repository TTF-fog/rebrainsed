use std::path::PathBuf;

pub fn get_ide_root_path(ide_path: &PathBuf, ide_name: &str) -> PathBuf {
    let mut new_path = ide_path.clone();
    new_path.pop();
    
    while let Some(last_component) = new_path.components().last() {
        println!("{}", last_component.as_os_str().to_str().unwrap());
        let last_str = last_component.as_os_str().to_str().unwrap();
        if last_str.contains(&ide_name.to_lowercase()) {
            break;
        }
        new_path.pop();
    }
    new_path
} 