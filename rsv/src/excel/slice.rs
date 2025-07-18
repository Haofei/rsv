use crate::args::Slice;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::constants::COMMA;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::reader::ExcelReader;
use rsv_lib::utils::util::werr_exit;
use rsv_lib::utils::writer::Writer;

impl Slice {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();
        // out file
        let out = new_path(path, "-slice").with_extension("csv");

        // open file
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut rdr = ExcelReader::new(path, self.sheet)?;

        // header
        if !self.no_header {
            let Some(r) = rdr.next() else { return Ok(()) };
            wtr.write_excel_line_unchecked(r, COMMA)
        }

        // slice
        match self.index {
            Some(index) => write_by_index(&mut rdr, &mut wtr, index),
            None => {
                let end = self
                    .end
                    .or_else(|| self.length.map(|l| self.start + l))
                    .unwrap_or(usize::MAX - 10);
                if self.start > end {
                    werr_exit!("Error: end index should be equal to or bigger than start index.");
                }
                write_by_range(&mut rdr, &mut wtr, self.start, end);
            }
        }

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}

fn write_by_index(rdr: &mut ExcelReader, wtr: &mut Writer, index: usize) {
    for r in rdr.iter().skip(rdr.next_called + index).take(1) {
        wtr.write_excel_line_unchecked(r, COMMA);
    }
}

fn write_by_range(rdr: &mut ExcelReader, wtr: &mut Writer, start: usize, end: usize) {
    for r in rdr.iter().skip(rdr.next_called + start).take(end - start) {
        wtr.write_excel_line_unchecked(r, COMMA);
    }
}
