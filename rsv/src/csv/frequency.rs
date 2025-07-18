use crate::args::Frequency;
use crossbeam_channel::bounded;
use dashmap::DashMap;
use rayon::prelude::*;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::file;
use rsv_lib::utils::filename;
use rsv_lib::utils::progress::Progress;
use rsv_lib::utils::reader::ChunkReader;
use rsv_lib::utils::util::print_frequency_table;
use std::thread;

impl Frequency {
    pub fn csv_run(&self) -> CliResult {
        let path = &self.path();

        // cols
        let col = Columns::new(&self.cols)
            .total_col_of(path, self.sep, self.quote)
            .parse();

        // open file and header
        let mut rdr = ChunkReader::new(path)?;
        let names: Vec<String> = if self.no_header {
            col.artificial_cols_with_appended_n()
        } else {
            let Some(r) = rdr.next() else { return Ok(()) };
            let r = r?;
            let r = self.split_row_to_vec(&r);
            if col.max >= r.len() {
                println!("[info] ignore a bad line # {r:?}!");
                col.artificial_cols_with_appended_n()
            } else {
                col.select_owned_vector_and_append_n(&r)
            }
        };

        // read file
        let (tx, rx) = bounded(1);
        thread::spawn(move || rdr.send_to_channel_by_chunks(tx, 10_000));

        // process
        let freq = DashMap::new();
        let mut prog = Progress::new();
        for task in rx {
            task.lines.par_iter().for_each(|r| {
                let r = self.split_row_to_vec(r);
                if col.max >= r.len() {
                    println!("[info] ignore a bad line # {r:?}!");
                } else {
                    let r = col.select_owned_string(&r);
                    *freq.entry(r).or_insert(0) += 1;
                }
            });

            if self.export {
                prog.add_chunks(1);
                prog.add_bytes(task.bytes);
                prog.print();
            }
        }

        let mut freq = freq.into_iter().collect::<Vec<(_, _)>>();
        if self.ascending {
            freq.sort_by(|a, b| a.1.cmp(&b.1));
        } else {
            freq.sort_by(|a, b| b.1.cmp(&a.1));
        }

        // apply head n
        if self.n > 0 {
            freq.truncate(self.n as usize)
        }

        // export or print
        if self.export {
            let new_path = filename::new_path(path, "-frequency");
            file::write_frequency_to_csv(&new_path, &names, freq);
            println!("\nSaved to file: {}", new_path.display());
        } else {
            print_frequency_table(&names, freq)
        }

        Ok(())
    }
}
