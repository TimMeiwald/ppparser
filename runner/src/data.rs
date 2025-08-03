use anyhow::{bail, Ok, Result};
use generator::GeneratedCode;
use std::fs::{self, create_dir};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

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
                println!("{e}");
                exit(1)
            }
            std::result::Result::Ok(_) => {
                println!("Path to Init {path:?}");
                println!("Step 2");

                let mut f = Command::new("cargo")
                    .arg("init")
                    .arg(path)
                    .spawn()
                    .expect("Failed to run Cargo Init");
                let _r = f.wait();
                println!("Cargo Dir: {path:?}");
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num")
                    .spawn()
                    .expect("Failed to add package num");
                let _r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num-derive")
                    .spawn()
                    .expect("Failed to add package num-derive");
                let _r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("num-traits")
                    .spawn()
                    .expect("Failed to add package num-traits");
                let _r = f.wait();
                let mut f = Command::new("cargo")
                    .current_dir(path)
                    .arg("add")
                    .arg("clap")
                    .arg("--features")
                    .arg("derive")
                    .spawn()
                    .expect("Failed to add package clap");
                let _r = f.wait();

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
        Ok(())
    }

    fn copy_dir_contents_to_dir(&self, target_dir: &Path, source_dir: &Path) -> Result<()> {
        if source_dir.is_dir() {
            for entry in fs::read_dir(source_dir)? {
                println!("{entry:?}");
                let e = &entry?.path();
                let file_name = e.file_name().expect("Should exist");
                let new_file_path = target_dir.join(file_name);

                println!("Creating File: {new_file_path:?}");
                let res = std::fs::File::create(&new_file_path);
                match res {
                    std::result::Result::Ok(_) => {}
                    Err(e) => {
                        println!("{e:?}")
                    }
                }
                println!("Attempting to copy contents from {e:?} to {new_file_path:?}");
                let _res = fs::copy(e, new_file_path)?;
            }

            Ok(())
        } else {
            panic!("Source Dir is not a directory!")
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
}
