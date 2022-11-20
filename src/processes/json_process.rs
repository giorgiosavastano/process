use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::fs;
use log::{info, warn};
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;
use std::iter::zip;

use crate::{process_trait::ProcessingCore, items::Item};
pub use super::JsonProcess;


impl ProcessingCore for JsonProcess {
    fn set_items(&mut self) -> Result<()> {

        // Open the file in read-only mode with buffer.
        let file = File::open(&self.json_items)?;
        let reader = BufReader::new(file);

        let items: Vec<Item> = serde_json::from_reader(reader)
        .expect("error while reading or parsing the json_items file");

        self.items = items;
        Ok(())
    }

    fn check_tmp_dir_exist(&self) -> Result<bool> {
        Ok(self.tmp_dir_path.is_some())
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