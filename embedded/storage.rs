use std::path::Path;
use walkdir::WalkDir;

pub struct FileCrypt<'key,'path>{
    key: Option<&'key str>,
    path: &'path Path,
}

pub struct DirCrypt<'key,'path>{
    key: Option<&'key str>,
    path: &'path Path,
}

fn list_all_files(dir: &Path,min_depth: usize,max_depth: usize) -> Vec<String> {
    WalkDir::new(dir)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect()
}

fn list_all_files_def(dir: &Path) -> Vec<String>{
    list_all_files(dir, 0, 2)
}

pub fn secrets_dir_get(path: &Path) -> Option<Vec<String>>{
    let files: Vec<String> = list_all_files_def(path);
    if files.is_empty(){
        return None;
    }
    Some(files)
}