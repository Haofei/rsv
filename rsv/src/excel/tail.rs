use crate::args::Tail;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::constants::COMMA;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::reader::ExcelReader;
use rsv_lib::utils::writer::Writer;

impl Tail {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();
        let out = new_path(path, "-tail").with_extension("csv");
        let mut wtr = Writer::file_or_stdout(self.export, &out)?;
        let mut range = ExcelReader::new(path, self.sheet)?;

        // header
        if !self.no_header {
            let Some(r) = range.next() else {
                return Ok(());
            };
            wtr.write_excel_line_unchecked(r, COMMA);
        }

        // show head n
        range
            .iter()
            .skip(range.next_called)
            .rev()
            .take(self.n)
            .rev()
            .for_each(|r| wtr.write_excel_line_unchecked(r, COMMA));

        if self.export {
            println!("Saved to file: {}", out.display())
        }

        Ok(())
    }
}
