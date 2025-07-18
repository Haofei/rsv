use crate::args::To;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::to::{excel_to_csv, is_valid_plain_text};

impl To {
    pub fn excel_run(&self) -> CliResult {
        let out = self.out.to_lowercase();
        let outsep = if out.ends_with("tsv") {
            '\t'.to_string()
        } else {
            ','.to_string()
        };

        if !is_valid_plain_text(&out) {
            let msg = format!("output file format of <{out}> is un-recognized.");
            return Err(msg.into());
        }

        excel_to_csv(&self.path(), self.sheet, &outsep, &out)?;

        Ok(())
    }
}
