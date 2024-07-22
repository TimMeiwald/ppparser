use anyhow::{Ok, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
pub struct DataGenerator {
    cache_dir: PathBuf,
    rules_dir: PathBuf,
    publisher_dir: PathBuf,
    generator_dir: PathBuf,
    runner_dir: PathBuf,
    target_dir: PathBuf,
}
impl DataGenerator {
    pub fn new(
        cache_dir: &str,
        rules_dir: &str,
        publisher_dir: &str,
        generator_dir: &str,
        runner_dir: &str,
        target_dir: PathBuf,
    ) -> Self {
        let dir_data = DataGenerator {
            cache_dir: PathBuf::from(cache_dir),
            rules_dir: PathBuf::from(rules_dir),
            publisher_dir: PathBuf::from(publisher_dir),
            generator_dir: PathBuf::from(generator_dir),
            runner_dir: PathBuf::from(runner_dir),
            target_dir: target_dir,
        };
        dir_data
            .verify()
            .unwrap_or_else(|_| panic!("One of the directories don't exist./n"));
        return dir_data;
    }

    fn verify(&self) -> Result<()> {
        self.cache_dir.canonicalize()?;
        self.rules_dir.canonicalize()?;
        self.publisher_dir.canonicalize()?;
        self.generator_dir.canonicalize()?;
        self.runner_dir.canonicalize()?;
        Ok(())
    }

    pub fn generate_data(&self) -> Result<()> {
        self.copy_all(&self.cache_dir)?;
        self.copy_all(&self.publisher_dir)?;
        self.copy_all(&self.generator_dir)?;
        self.copy_all(&self.runner_dir)?;
        self.copy_all(&self.rules_dir)?;
        Ok(())
    }

    fn copy_all(&self, pathbuf: &PathBuf) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter().last());
        copy_dir_all(&self.cache_dir, cache_target)
    }
}
