use anyhow::{Context, Result};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::{fs, io};
use log::{info, warn};
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use std::iter::zip;

use crate::{process_trait::ProcessingCore, items::Item};
pub use super::Process;


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

    fn check_tmp_dir_exist(&self) -> Result<bool> {
        Ok(self.tmp_dir_path.is_some())
    }

    fn create_tmp_directory(&self) -> Result<()> {
        fs::create_dir_all(self.tmp_dir_path.as_ref().context("as_ref() failed")?).with_context(
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
            }
        Ok(true)
    }
}