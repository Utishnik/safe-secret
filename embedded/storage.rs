use std::path::Path;
use std::path::PathBuf;
use std::{fs, iter};
use walkdir::WalkDir;

const DEFAULT_MIN_DEPTH: usize = 0;
const DEFAULT_MAX_DEPTH: usize = 2;

pub struct FileCrypt<'key, 'path> {
    crypt_data: Vec<u8>,
    decrypt_data: Vec<u8>,
    key: Option<&'key str>,
    path: &'path Path,
}

impl FileCrypt<'_, '_> {
    pub fn set_crypt_data_file(&mut self) {
        let get_bytes_crypt_data: Result<Vec<u8>, std::io::Error> = load_crypt_file(self.path);
        if get_bytes_crypt_data.is_err() {
            eprintln!("set_crypt_data_file is err!");
            return;
        }
        self.crypt_data = get_bytes_crypt_data.unwrap();
    }

    pub fn get_crypt_data_file_borrow(&self) -> &Vec<u8> {
        &self.crypt_data
    }

    pub fn clone_crypt_data_file(&mut self) -> Vec<u8> {
        self.crypt_data.clone()
    }

    pub fn move_crypt_data_file(self) -> Vec<u8> {
        self.crypt_data
    }

    pub fn get_decrypt_data_file_borrow(&self) -> &Vec<u8> {
        &self.decrypt_data
    }

    pub fn clone_decrypt_data_file(&mut self) -> Vec<u8> {
        self.decrypt_data.clone()
    }
}

pub struct DirCrypt<'key, 'path> {
    data: Vec<FileCrypt<'key, 'path>>,
    key: Option<&'key str>,
    path: &'path Path,
}

fn list_all_files(dir: &Path, min_depth: usize, max_depth: usize) -> Vec<String> {
    WalkDir::new(dir)
        .min_depth(min_depth)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect()
}

fn list_all_files_def(dir: &Path) -> Vec<String> {
    list_all_files(dir, DEFAULT_MIN_DEPTH, DEFAULT_MAX_DEPTH)
}

pub fn secrets_dir_get(path: &Path) -> Option<Vec<String>> {
    let files: Vec<String> = list_all_files_def(path);
    if files.is_empty() {
        return None;
    }
    Some(files)
}

type FileBytes = Vec<u8>;

fn load_crypt_file(path: &Path) -> std::io::Result<FileBytes> {
    let bytes: FileBytes = fs::read(path)?;
    Ok(bytes)
}

fn str_to_path<'a>(str_path: &'a str) -> &'a Path {
    Path::new(str_path)
}

fn str_to_owned_path<S: Into<PathBuf>>(str_path: S) -> PathBuf {
    str_path.into()
}

pub fn load_crypt_dir(path: &Path) -> Option<Vec<FileBytes>> {
    let ls: Option<Vec<String>> = secrets_dir_get(path);
    if ls.is_none() {
        return None;
    }
    let unwrap_ls: Vec<String> = ls.unwrap();
    let unwrap_ls_len: usize = unwrap_ls.len();
    let mut res: Vec<FileBytes> = Vec::with_capacity(unwrap_ls_len);
    for item in res.iter_mut() {
        let ls_get: Option<&String> = unwrap_ls.get(0);
        if ls_get.is_none() {
            println!("load_crypt_dir error unwrap_ls index: ");
            return None;
        }
        let ls_get_convert: &Path = str_to_path(ls_get.unwrap());
        let get: Result<Vec<u8>, std::io::Error> = load_crypt_file(ls_get_convert);
        if get.is_err() {
            println!("load_crypt_dir error load_crypt_file index: ");
            return None;
        }
        let get_unwrap: Vec<u8> = get.unwrap();
        *item = get_unwrap;
    }
    Some(res)
}
