#![crate_name = "processing_chain"]

//! The `processing-chain` crate provides a convenient way to seamlessly set up processing
//! chains for large amounts of data.
//!

use anyhow::{Context, Ok, Result};
use indicatif::ParallelProgressIterator;
use log::{info, warn};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, io};
use std::iter::{zip};

/// An item.
///
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub input_item_paths: Vec<PathBuf>,
    pub output_item_paths: Vec<PathBuf>,
    pub tmp_item_paths: Option<Vec<PathBuf>>,
}

/// A process.
///
#[derive(Debug, Default)]
pub struct Process {
    pub name: String,
    pub inputs_dir_path: PathBuf,
    pub inputs_extenion: String,
    pub outputs_dir_path: PathBuf,
    pub tmp_dir_path: Option<PathBuf>,
    pub overwrite: bool,
    pub items: Vec<Item>,
}

/// Processing trait.
///
pub trait ProcessingCore {
    fn set_items(&mut self) -> Result<()>;
    fn check_all_inputs_exist(&self) -> Result<bool>;
    fn create_tmp_directory(&self) -> Result<()>;
    fn process_items<F>(&self, f: F) -> Result<bool>
    where
        F: Fn(&Item) -> Result<bool> + Send + Sync;
    fn move_files(&self) -> Result<bool>;
}

impl ProcessingCore for Process {
    fn set_items(&mut self) -> Result<()> {
        let entries = fs::read_dir(&self.inputs_dir_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut items: Vec<Item> = Vec::new();
        let mut i = 0;
        for e in entries.into_iter() {
            if e.extension().unwrap_or_else(|| OsStr::new("")) == OsStr::new(&self.inputs_extenion)
            {
                let file_name = e.file_name().context("file_name() failed")?;
                let mut output_item_path = PathBuf::new();
                let mut tmp_item_path = PathBuf::new();

                if self.tmp_dir_path.is_some() {
                    tmp_item_path.push(
                        self.tmp_dir_path
                            .as_ref()
                            .context("as_ref() failed")?
                            .to_path_buf()
                            .join(file_name),
                    );
                    output_item_path.push(self.outputs_dir_path.to_path_buf().join(file_name));
                } else {
                    output_item_path.push(self.outputs_dir_path.to_path_buf().join(file_name));
                }

                if !self.overwrite && output_item_path.exists() {
                    continue;
                }

                let it = Item {
                    name: format!("file_{}", i),
                    input_item_paths: vec![e.to_path_buf()],
                    output_item_paths: vec![output_item_path],
                    tmp_item_paths: Some(vec![tmp_item_path]),
                };
                i += 1;
                items.push(it)
            }
        }
        self.items = items;
        Ok(())
    }

    fn check_all_inputs_exist(&self) -> Result<bool> {
        let mut test = true;
        for f in self.items.iter() {
            for i in f.input_item_paths.iter() {
                test = test && i.exists();
            }
        }
        Ok(test)
    }

    fn create_tmp_directory(&self) -> Result<()> {
        fs::create_dir_all(&self.tmp_dir_path.as_ref().context("as_ref() failed")?).with_context(
            || {
                format!(
                    "could not create temporary directory `{}`",
                    &self.tmp_dir_path.as_ref().unwrap().display()
                )
            },
        )?;
        info! {"Created tmp directory at {}",  &self.tmp_dir_path.as_ref().context("as_ref() failed")?.display()}
        Ok(())
    }

    fn process_items<F>(&self, f: F) -> Result<bool>
    where
        F: Fn(&Item) -> Result<bool> + Send + Sync,
    {
        info!("Process name {}", self.name);

        (self.items)
            .par_iter()
            .progress_count(self.items.len() as u64)
            .for_each(|i| {
                let fl = f(i)
                    .with_context(|| format!("could not process item `{}`", i.name))
                    .unwrap();

                if !fl {
                    let warn_description = format!("Process for {} not succesfull.", i.name);
                    warn!("Warning! {}!", warn_description);
                }
            });

        Ok(true)
    }

    fn move_files(&self) -> Result<bool> {
        for i in self.items.iter() {

            let tmp_paths = i.tmp_item_paths.as_ref().unwrap();

            let iter = zip(tmp_paths, &i.output_item_paths);

            for (p1, p2 )in iter {
                fs::rename(p1, p2)?;
            }



            // Zip::from(i.tmp_item_paths.unwrap().iter())
            // .and(i.output_item_paths.par_iter())
            //     .for_each(|(x, y)| { fs::rename(
            //                         x,
            //                         y,).unwrap();}
            //                     );
            }
        Ok(true)
    }
}

pub fn run_process<F>(mut proc: Process, f: F) -> Result<Process>
where
    F: Fn(&Item) -> Result<bool> + Send + Sync,
{
    proc.set_items()?;

    if proc.check_all_inputs_exist()? {
        println!("All good!");
    }

    if proc.tmp_dir_path.is_some() {
        proc.create_tmp_directory()?;
    }

    if proc.process_items(f)? {
        println!("All file processed!");
    }

    if proc.tmp_dir_path.is_some() {
        proc.move_files()?;
    }

    Ok(proc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn _process_item(item: &Item) -> Result<bool> {
        // define how to process a single item
        info!(
            "Processing {} {:?} -> {:?}",
            item.name, item.input_item_paths, item.output_item_paths
        );
        Ok(true)
    }

    #[test]
    fn process_default_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            ..Process::default()
        };
        assert_eq!(proc.overwrite, false);
        assert_eq!(proc.tmp_dir_path, None);
        assert_eq!(proc.inputs_extenion, "toml");
        assert_eq!(proc.outputs_dir_path.to_str().unwrap(), "");
    }

    #[test]
    fn run_process_items_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            outputs_dir_path: PathBuf::from("Test"),
            ..Process::default()
        };

        let proc = run_process(proc, _process_item).unwrap();
        let first_item = proc.items.first().unwrap();
        assert_eq!(first_item.name, "file_0");
        assert_eq!(
            first_item.input_item_paths.first().unwrap().file_name().unwrap(),
            "Cargo.toml"
        );
        assert_eq!(first_item.input_item_paths.first().unwrap().extension().unwrap(), "toml");
        assert_eq!(first_item.output_item_paths.first().unwrap().extension().unwrap(), "toml");
    }

    #[test]
    fn run_process_empty_items_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            outputs_dir_path: env::current_dir().unwrap(),
            ..Process::default()
        };

        let proc = run_process(proc, _process_item).unwrap();
        assert!(proc.items.is_empty());
    }
}
