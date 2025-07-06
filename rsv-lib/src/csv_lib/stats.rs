use crate::utils::chunk::{parse_chunk, ChunkResult};
use crate::utils::column::Columns;
use crate::utils::column_stats::{CStat, ColumnStats};
use crate::utils::column_type::ColumnTypes;
use crate::utils::file::column_n;
use crate::utils::progress::Progress;
use crate::utils::reader::ChunkReader;
use crate::utils::return_result::{CliResultData, ResultData};
use crate::utils::row_split::CsvRowSplitter;
use crossbeam_channel::{bounded, unbounded};
use rayon::ThreadPoolBuilder;
use std::path::PathBuf;

pub fn csv_stats(
    file: &PathBuf,
    sep: char,
    quote: char,
    no_header: bool,
    cols: String,
) -> CliResultData {
    let mut result_data = ResultData::new();
    result_data.insert_header(CStat::get_fields.iter().map(|f| f.to_string()).collect());

    // Column
    let cols = Columns::new(cols.as_str())
        .total_col_of(file, sep, quote)
        .parse();
    let Some(col_type) = ColumnTypes::guess_from_csv(file, sep, quote, no_header, &cols)? else {
        return Ok(Some(result_data));
    };

    // open file
    let mut rdr = ChunkReader::new(file)?;

    // header
    let name = if no_header {
        let Some(n) = column_n(file, sep, quote)? else {
            return Ok(Some(result_data));
        };
        cols.artificial_n_cols(n)
    } else {
        let Some(r) = rdr.next() else {
            return Ok(Some(result_data));
        };
        CsvRowSplitter::new(&r?, sep, quote)
            .map(|i| i.to_owned())
            .collect::<Vec<_>>()
    };

    // stats holder
    let mut stat = ColumnStats::new(&col_type, &name);
    let empty_stat = stat.clone();

    // parallel channels
    let (tx_chunk, rx_chunk) = bounded(2);
    let (tx_chunk_n_control, rx_chunk_n_control) = bounded(200);
    let (tx_result, rx_result) = unbounded();

    // progress bar
    let mut prog = Progress::new();

    // threadpool
    let pool = ThreadPoolBuilder::new().build().unwrap();

    // read
    pool.spawn(move || rdr.send_to_channel_by_chunks(tx_chunk, 50_000));

    // parallel process
    pool.scope(|s| {
        // add chunk to threadpool for process
        s.spawn(|_| {
            for task in rx_chunk {
                tx_chunk_n_control.send(()).unwrap();

                let tx = tx_result.clone();
                let st = empty_stat.clone();
                let sep_inner = sep;
                let quote_inner = quote;
                // println!("dispatch........");
                pool.spawn(move || parse_chunk(task, tx, st, sep_inner, quote_inner));
            }

            drop(tx_result);
            drop(tx_chunk_n_control);
        });

        // receive result
        for ChunkResult { bytes, stat: o } in rx_result {
            rx_chunk_n_control.recv().unwrap();
            // println!("result-----------");
            // this is bottleneck, merge two hashset is very slow.
            stat.merge(o);

            prog.add_bytes(bytes);
            prog.add_chunks(1);
            prog.print();
        }

        prog.clear();
    });

    // refine result
    stat.cal_unique_and_mean();
    result_data.insert_records(stat.stat.iter().map(|s| s.get_fields_values()));
    // stat.print();

    // println!("Total rows: {}", stat.rows);
    prog.print_elapsed_time();

    Ok(Some(result_data))
}
