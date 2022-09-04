#![crate_name = "processing_chain"]

//! The `processing-chain` crate provides a convenient way to seamlessly set up processing
//! chains for large amount of data.
//!

use anyhow::{Context, Result};
use log::{info, warn};
use rayon::prelude::*;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, io};

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub input_item_path: PathBuf,
    pub output_item_path: PathBuf,
}

#[derive(Debug)]
pub struct Process {
    pub name: String,
    pub inputs_dir_path: PathBuf,
    pub inputs_extenion: String,
    pub outputs_dir_path: PathBuf,
    pub tmp_dir_path: PathBuf,
    pub overwrite: bool,
    pub items: Vec<Item>,
}

pub trait ProcessingCore {
    fn set_items(self: &mut Self) -> Result<()>;
    fn check_all_inputs_exist(self: &Self) -> Result<bool>;
    fn create_tmp_directory(self: &Self) -> Result<()>;
    fn process_items<F>(self: &Self, f: F) -> Result<bool>
    where
        F: Fn(&Item) -> Result<bool> + Send + Sync;
}

impl ProcessingCore for Process {
    fn set_items(&mut self) -> Result<()> {
        let entries = fs::read_dir(&self.inputs_dir_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        let mut items: Vec<Item> = Vec::new();
        let mut i = 0;
        for e in entries.into_iter() {
            if e.extension().unwrap_or(OsStr::new("default")) == OsStr::new(&self.inputs_extenion) {
                let file_name = e.file_name().unwrap();
                let mut output_item_path = PathBuf::new();

                if self.tmp_dir_path.to_str() != Some("default") {
                    output_item_path.push(self.tmp_dir_path.to_path_buf().join(file_name));
                } else {
                    output_item_path.push(self.outputs_dir_path.to_path_buf().join(file_name));
                }

                let it = Item {
                    name: format!("file_{}", i),
                    input_item_path: e.to_path_buf(),
                    output_item_path: output_item_path,
                };
                i += 1;
                items.push(it)
            }
        }
        self.items = items;
        Ok(())
    }

    fn check_all_inputs_exist(self: &Self) -> Result<bool> {
        let mut test = true;
        for f in self.items.iter() {
            test = test && f.input_item_path.exists();
        }
        Ok(test)
    }

    fn create_tmp_directory(self: &Self) -> Result<()> {
        fs::create_dir_all(&self.tmp_dir_path).with_context(|| {
            format!(
                "could not create temporary directory `{}`",
                &self.tmp_dir_path.display()
            )
        })?;
        info! {"Created tmp directory at {}",  &self.tmp_dir_path.display()}
        Ok(())
    }

    fn process_items<F>(self: &Self, f: F) -> Result<bool>
    where
        F: Fn(&Item) -> Result<bool> + Send + Sync,
    {
        info!("Process name {}", self.name);

        (self.items).par_iter().for_each(|i| {
            let fl = f(i)
                .with_context(|| format!("could not process item `{}`", i.name))
                .unwrap();

            if fl == false {
                let warn_description = format!("Process for {} not succesfull.", i.name);
                warn!("Warning! {}!", warn_description);
            }
        });

        Ok(true)
    }
}

fn _move_files(src: PathBuf, dst: PathBuf) -> Result<()> {
    fs::rename(src, dst)?;
    Ok(())
}
