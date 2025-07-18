use crate::args::Sort;
use rsv_lib::utils::sort::SortColumns;
use rsv_lib::utils::writer::Writer;
use rsv_lib::utils::{cli_result::CliResult, filename::new_file};
use std::io::{stdin, BufRead};

impl Sort {
    pub fn io_run(&self) -> CliResult {
        // rdr and wtr
        let mut rdr = stdin().lock().lines();
        let out = new_file("sorted.csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;

        // cols
        let cols = SortColumns::from(&self.cols)?;

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            wtr.write_str_unchecked(r?)
        }

        // lines
        let lines = rdr.filter_map(|i| i.ok()).collect::<Vec<_>>();

        // sort
        cols.sort_and_write(&lines, self.sep, self.quote, &mut wtr)?;

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
