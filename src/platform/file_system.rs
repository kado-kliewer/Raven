use std::fs;
use std::path::Path;
use std::io;
pub struct FileSystem {

}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {}
    }

    pub fn read_file(&self, path: &Path) -> io::Result<String> {
        fs::read_to_string(path)
    }

    pub fn write_file(&self, path: &Path, contents: &str) -> io::Result<()> {
        fs::write(path, contents)
    }

    pub fn read_binary(&self, path: &Path) -> io::Result<Vec<u8>> {
        fs::read(path)
    }

    pub fn write_binary(&self, path: &Path, contents: &[u8]) -> io::Result<()> {
        fs::write(path, contents)
    }

    pub fn create_directory(&self, path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    pub fn delete_file(&self, path: &Path) -> io::Result<()> {
        fs::remove_file(path)
    }

    pub fn file_exists(&self, path: &Path) -> bool {
        path.exists() && path.is_file()
    }

    pub fn directory_exists(&self, path: &Path) -> bool {
        path.exists() && path.is_dir()
    }
}