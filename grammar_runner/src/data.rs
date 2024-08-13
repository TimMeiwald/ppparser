use anyhow::{bail, Ok, Result};
use std::fs::{self, create_dir, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::str::FromStr;
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

pub fn copy_file(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<u64> {
    let f = fs::copy(src, dst)?;
    Ok(f)
}

pub struct DataGenerator {
    cache_dir: PathBuf,
    rules_dir: PathBuf,
    publisher_dir: PathBuf,
    // generator_dir: PathBuf,
    // runner_dir: PathBuf,
    target_dir: PathBuf,
    parser_core_dir: PathBuf,
    source: PathBuf,
}
impl DataGenerator {
    pub fn new(
        cache_dir: &str,
        rules_dir: &str,
        publisher_dir: &str,
        // generator_dir: &str,
        // runner_dir: &str,
        target_dir: PathBuf,
        parser_core_dir: &str,
        source: PathBuf,
    ) -> Self {
        let dir_data = DataGenerator {
            cache_dir: PathBuf::from(cache_dir),
            rules_dir: PathBuf::from(rules_dir),
            publisher_dir: PathBuf::from(publisher_dir),
            // generator_dir: PathBuf::from(generator_dir),
            // runner_dir: PathBuf::from(runner_dir),
            parser_core_dir: PathBuf::from(parser_core_dir),
            target_dir,
            source,
        };
        dir_data
            .verify()
            .unwrap_or_else(|_| panic!("One of the directories don't exist./n"));
        dir_data
    }

    fn verify(&self) -> Result<()> {
        self.cache_dir.canonicalize()?;
        self.rules_dir.canonicalize()?;
        self.publisher_dir.canonicalize()?;
        // self.generator_dir.canonicalize()?;
        // self.runner_dir.canonicalize()?;
        self.parser_core_dir.canonicalize()?;

        Ok(())
    }

    pub fn generate_parser(&self) -> Result<()> {
        let parser = grammar_generator::generate_parser(&self.source);
        match parser {
            Some(p) => {
                let rules_enum = p.rules_enum;
                let rules = p.rules;
                let mut f =
                    self.make_file(&PathBuf::from_str("./grammar_parser/src/lib.rs").unwrap())?;
                let s2 = "use cache::*;
use parser_core::*;
use publisher::*;
use rules::Rules;

pub use parser_core::Context;
pub use parser_core::Source;";
                f.write_all(s2.as_bytes())?;
                for rule in rules {
                    f.write_all(rule.as_bytes())?;
                }
                self.remove_file(&PathBuf::from_str("./rules/src/rules.rs").unwrap())?;
                let mut f2 = self.make_file(&PathBuf::from_str("./rules/src/rules.rs").unwrap())?;

                let header = "use num_derive::FromPrimitive;
#[derive(FromPrimitive, Clone, Copy, Debug)]";
                f2.write_all(header.as_bytes())?;
                f2.write_all(rules_enum.as_bytes())?;

                Ok(())
            }
            None => {
                bail!("Failed to generate parser");
            }
        }
    }

    pub fn generate_data(&self) -> Result<()> {
        self.copy_all(&self.cache_dir)?;
        self.copy_all(&self.publisher_dir)?;
        // self.copy_all(&self.generator_dir)?;
        // self.copy_all(&self.runner_dir)?;
        self.copy_all(&self.rules_dir)?;
        self.copy_all(&self.parser_core_dir)?;
        self.copy_all(&PathBuf::from_str("./parser").unwrap())?;

        self.copy_file(&PathBuf::from_str("Cargo.toml").unwrap())?;
        self.copy_file_with_dst(
            &PathBuf::from_str("./data/Cargo.toml").unwrap(),
            &PathBuf::from_str("Cargo.toml").unwrap(),
        )?;
        self.copy_file(&PathBuf::from_str("Cargo.lock").unwrap())?;
        self.copy_file(&PathBuf::from_str(".gitignore").unwrap())?;
        self.create_dir(&PathBuf::from_str("./grammar_parser/").unwrap())?;
        self.create_dir(&PathBuf::from_str("./grammar_parser/src/").unwrap())?;
        self.copy_file(&PathBuf::from_str("./grammar_parser/Cargo.toml").unwrap())?;
        self.generate_parser()?;
        Ok(())
    }

    fn copy_all(&self, pathbuf: &Path) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter());
        println!("{:?}", cache_target);
        copy_dir_all(pathbuf, cache_target)
    }
    fn copy_file(&self, pathbuf: &Path) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter());
        println!("{:?}", cache_target);
        let success = copy_file(pathbuf, cache_target);
        match success {
            std::result::Result::Ok(_) => Ok(()),
            Err(e) => {
                bail!("Failed to create {:?}: {:?}", pathbuf, e)
            }
        }
    }

    fn copy_file_with_dst(&self, pathbuf: &Path, dst: &Path) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(dst.iter());
        println!("{:?}", cache_target);
        let success = copy_file(pathbuf, cache_target);
        match success {
            std::result::Result::Ok(_) => Ok(()),
            Err(e) => {
                bail!("Failed to create {:?}: {:?}", pathbuf, e)
            }
        }
    }

    fn make_file(&self, pathbuf: &Path) -> Result<File> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter());
        println!("{:?}", cache_target);
        let success = std::fs::File::create(cache_target);
        match success {
            std::result::Result::Ok(f) => Ok(f),
            Err(e) => {
                bail!("Failed to create {:?}: {:?}", pathbuf, e)
            }
        }
    }
    fn create_dir(&self, pathbuf: &Path) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter());
        let f = cache_target.clone();
        let success = create_dir(cache_target);
        match success {
            std::result::Result::Ok(()) => Ok(()),
            Err(e) => {
                bail!("Failed to create {:?}: {:?}", f, e)
            }
        }
    }
    fn remove_file(&self, pathbuf: &Path) -> Result<()> {
        let cache_target = &mut self.target_dir.clone();
        cache_target.extend(pathbuf.iter());
        println!("{:?}", cache_target);
        let success = std::fs::remove_file(cache_target);
        match success {
            std::result::Result::Ok(_) => Ok(()),
            Err(e) => {
                bail!("Failed to create {:?}: {:?}", pathbuf, e)
            }
        }
    }
}
