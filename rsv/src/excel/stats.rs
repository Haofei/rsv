use crate::args::Stats;
use rsv_lib::utils::cli_result::CliResult;
use rsv_lib::utils::column::Columns;
use rsv_lib::utils::column_stats::ColumnStats;
use rsv_lib::utils::column_type::ColumnTypes;
use rsv_lib::utils::filename::new_path;
use rsv_lib::utils::reader::ExcelReader;
use std::fs::File;
use std::io::{BufWriter, Write};

impl Stats {
    pub fn excel_run(&self) -> CliResult {
        let path = &self.path();

        // read file
        let mut rdr = ExcelReader::new(path, self.sheet)?;

        // Column type
        let cols = Columns::new(&self.cols).total_col(rdr.column_n()).parse();
        let col_type = ColumnTypes::guess_from_excel(&rdr, self.no_header, &cols).unwrap();

        // header
        let name = match self.no_header {
            true => cols.artificial_n_cols(rdr.column_n()),
            false => {
                let Some(r) = rdr.next() else { return Ok(()) };
                r.iter().map(|i| i.to_string()).collect::<Vec<_>>()
            }
        };

        // stats holder
        let mut stat = ColumnStats::new(&col_type, &name);

        // read
        rdr.iter()
            .skip(rdr.next_called)
            .for_each(|r| stat.parse_excel_row(r));

        // refine result
        stat.cal_unique_and_mean();

        // print
        if self.export {
            let out = new_path(path, "-stats").with_extension("csv");
            let mut wtr = BufWriter::new(File::create(&out)?);
            wtr.write_all(stat.to_string().as_bytes())?;
            println!("Saved to file: {}", out.display());
        } else {
            stat.print();
        }

        println!("Total rows: {}", stat.rows);

        Ok(())
    }
}
