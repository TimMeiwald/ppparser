use anyhow::{bail, Ok, Result};
use generator::GeneratedCode;
use std::fs::{self, create_dir, File};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};
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
    name: String,
    target_dir: PathBuf,
    parser_core_dir: PathBuf,
    source: PathBuf,
}
impl DataGenerator {
    pub fn new(target_dir: PathBuf, parser_core_dir: &str, source: PathBuf, name: String) -> Self {
        let dir_data = DataGenerator {
            name,
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
        self.parser_core_dir.canonicalize()?;

        Ok(())
    }

    pub fn generate_parser(&self) -> GeneratedCode<'_> {
        let parser = generator::generate_parser(&self.source);
        match parser {
            None => panic!("Failed to generate parser!"),
            Some(parser) => parser,
        }
    }

    pub fn generate_data(&self) -> Result<()> {
        println!("Step 1");
        let parser = self.generate_parser();
        let path = &self.target_dir.canonicalize().unwrap().join(&self.name);
        let r = self.create_dir(path);

        match r {
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
            std::result::Result::Ok(_) => {
                println!("Path to Init {:?}", path);
                println!("Step 2");

                let mut f = Command::new("cargo")
                    .arg("init")
                    .arg(path)
                    .spawn()
                    .expect("Failed to run Cargo Init");
                let r = f.wait();
                println!("Cargo Dir: {:?}", path);
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num")
                    .spawn()
                    .expect("Failed to add package num");
                let r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num-derive")
                    .spawn()
                    .expect("Failed to add package num-derive");
                let r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num-traits")
                    .spawn()
                    .expect("Failed to add package num-traits");
                let r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("clap")
                    .arg("--features")
                    .arg("derive")
                    .spawn()
                    .expect("Failed to add package clap");
                let r = f.wait();

                let copy_folder = self.parser_core_dir.join("src");
                let path = path.join("src");
                println!("{:?} {:?}", self.source, path);
                let _ = self.copy_dir_contents_to_dir(&path, &copy_folder);
                let rules_enum_file = path.join("rules.rs");
                let parser_rs = path.join("parser.rs");
                let sample_main = path.join("main.rs");
                let _ = fs::write(rules_enum_file, parser.rules_enum_file_content());
                let _ = fs::write(parser_rs, parser.parser_file_content());
                let header = parser.sample_main_header(&self.name);
                let content = fs::read(copy_folder.join("sample_main.txt"))?;
                let sample_main_content = header + &String::from_utf8(content)?;
                let _ = fs::write(sample_main, sample_main_content);
                fs::remove_file(path.join("sample_main.txt"))?;
            }
        }

        // println!("Step 3");

        // self.copy_all(&self.parser_core_dir)?;
        // self.copy_all(&PathBuf::from_str("./parser").unwrap())?;

        // self.copy_file(&PathBuf::from_str("Cargo.toml").unwrap())?;
        // self.copy_file_with_dst(
        //     &PathBuf::from_str("./data/Cargo.toml").unwrap(),
        //     &PathBuf::from_str("Cargo.toml").unwrap(),
        // )?;
        // self.copy_file(&PathBuf::from_str("Cargo.lock").unwrap())?;
        // self.copy_file(&PathBuf::from_str(".gitignore").unwrap())?;
        // self.create_dir(&PathBuf::from_str("./parser/").unwrap())?;
        // self.create_dir(&PathBuf::from_str("./parser/src/").unwrap())?;
        // self.copy_file(&PathBuf::from_str("./parser/Cargo.toml").unwrap())?;
        Ok(())
    }

    fn copy_dir_contents_to_dir(&self, target_dir: &PathBuf, source_dir: &PathBuf) -> Result<()> {
        if source_dir.is_dir() {
            for entry in fs::read_dir(source_dir)? {
                println!("{:?}", entry);
                let e = &entry?.path();
                let file_name = e.file_name().expect("Should exist");
                let new_file_path = target_dir.join(file_name);

                println!("Creating File: {:?}", new_file_path);
                let res = std::fs::File::create(&new_file_path);
                match res {
                    std::result::Result::Ok(_) => {}
                    Err(e) => {
                        println!("{:?}", e)
                    }
                }
                println!(
                    "Attempting to copy contents from {:?} to {:?}",
                    e, new_file_path
                );
                let res = fs::copy(e, new_file_path)?;
            }

            Ok(())
        } else {
            panic!("Source Dir is not a directory!")
        }
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
                if e.kind() != ErrorKind::AlreadyExists {
                    bail!("Failed to create {:?}: {:?}", f, e)
                } else {
                    Ok(())
                }
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
