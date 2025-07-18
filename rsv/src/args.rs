use clap::Args;
use rsv_lib::utils::{row_split::CsvRowSplitter, util::get_valid_sep};

#[derive(Debug, Args)]
pub struct Count {
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Get the nth worksheet of Excel file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Size {
    /// File to open
    pub filename: Option<String>,
}

#[derive(Debug, Args)]
pub struct Estimate {
    /// File to open
    pub filename: Option<String>,
    /// Get the nth worksheet for an Excel file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Head {
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Number of records to show
    #[arg(short, long, default_value_t = 10)]
    pub n: usize,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Export to a file named current-file-head.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Field separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
}

#[derive(Debug, Args)]
pub struct Tail {
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Number of records to show
    #[arg(short, long, default_value_t = 10)]
    pub n: usize,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Export to a file named current-file-head.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
}

#[derive(Debug, Args)]
pub struct Headers {
    /// File to open
    pub filename: Option<String>,
    /// Field separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Clean {
    /// File to open
    pub filename: Option<String>,
    /// Output file, default to current-file-cleaned.csv
    #[arg(short, long, default_value_t = String::from(""), hide_default_value=true)]
    pub output: String,
    /// Escape char to clean
    #[arg(short, long, default_value_t = String::from("\""))]
    pub escape: String,
}

#[derive(Debug, Args)]
pub struct Flatten {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Line delimiter for printing
    #[arg(short, long, default_value_t = String::from("#"))]
    pub delimiter: String,
    /// Number of records to show, n=-1 to show all
    #[arg(short, long, default_value_t = 5)]
    pub n: i32,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Frequency {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Columns to generate frequency table
    #[arg(short, long, default_value_t = String::from("0"), allow_hyphen_values=true)]
    pub cols: String,
    /// Ascending order or not
    #[arg(short, long, default_value_t = false)]
    pub ascending: bool,
    /// Export result to a frequency.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Top N to keep in frequency table
    #[arg(short, long, default_value_t = -1)]
    pub n: i32,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Split {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Column to split upon
    #[arg(short, long, default_value_t = 0)]
    pub col: usize,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Number of records to write in each separate file
    #[arg(long)]
    pub size: Option<usize>,
}

#[derive(Debug, Args)]
pub struct Slice {
    /// File to open
    pub filename: Option<String>,
    /// Start index of CSV
    #[arg(short, long, default_value_t = 0)]
    pub start: usize,
    /// End index of CSV
    #[arg(short, long)]
    pub end: Option<usize>,
    /// Slice length
    #[arg(short, long)]
    pub length: Option<usize>,
    /// Index for single record of CSV
    #[arg(short, long)]
    pub index: Option<usize>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Export data to a current-file-slice.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Select {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Separator
    #[arg(short, long, default_value_t = ',')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Columns to select, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
    #[arg(short, long, default_value_t = String::from(""), allow_hyphen_values=true)]
    pub cols: String,
    /// Row filter, support syntax 0=a,b,c or 0=a,b&1=c,d; Default to None
    #[arg(short, long, default_value_t = String::from(""), allow_hyphen_values=true)]
    pub filter: String,
    /// Export results to a file named current-file-selected.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Stats {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote Char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Columns to generate statistics, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
    #[arg(short, long, default_value_t = String::from(""), allow_hyphen_values=true)]
    pub cols: String,
    /// Export results to a file named current-file-selected.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Excel2csv {
    /// File to open
    pub filename: Option<String>,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Separator Char
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote Char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
}

#[derive(Debug, Args)]
pub struct Table {
    /// File to open
    pub filename: Option<String>,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Separator Char
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote Char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
}

#[derive(Debug, Args)]
pub struct Search {
    /// Regex pattern to search
    pub pattern: String,
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Separator Char
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote Char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Search specific columns, e.g. -f=0,1 to search first two columns; Default to all columns
    #[arg(short, long, default_value_t = String::from(""), allow_hyphen_values=true)]
    pub filter: String,
    /// Columns to select in output, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
    #[arg(short, long, default_value_t = String::from(""), allow_hyphen_values=true)]
    pub cols: String,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = String::from("0"), allow_hyphen_values = true)]
    pub sheet: String,
    /// Export to a file named current-file-searched.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
}

#[derive(Debug, Args)]
pub struct Sort {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t=',', value_parser=get_valid_sep )]
    pub sep: char,
    /// Quote Char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Columns to sort by, support syntax 0 (first column),
    /// "-0" (descending), "-0N" (as numeric) or "0N,-1" (two columns)
    #[arg(short, long, default_value_t = String::from("0"), allow_hyphen_values=true)]
    pub cols: String,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Export to a file named current-file-searched.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
}

#[derive(Debug, Args)]
pub struct Sample {
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Sample size
    #[arg(short, long, default_value_t = 10)]
    pub n: usize,
    /// Get the nth worksheet of EXCEL file
    #[arg(long)]
    pub seed: Option<usize>,
    /// Export to a file named current-file-searched.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
    /// Show line number
    #[arg(long, long, default_value_t = false)]
    pub show_number: bool,
    /// Time limit
    #[arg(short, long, default_value_t = 0.0)]
    pub time_limit: f32,
}

#[derive(Debug, Args)]
pub struct To {
    /// Output file, a file name or a file format
    pub out: String,
    /// File to open
    pub filename: Option<String>,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Input file Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Quote char
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
}

#[derive(Debug, Args)]
pub struct Unique {
    /// File to open
    pub filename: Option<String>,
    /// Separator
    #[arg(short, long, default_value_t = ',', value_parser=get_valid_sep)]
    pub sep: char,
    /// Separator
    #[arg(short, long, default_value_t = '"')]
    pub quote: char,
    /// Whether the file has a header
    #[arg(long, default_value_t = false)]
    pub no_header: bool,
    /// Columns to filter
    #[arg(short, long, default_value_t = String::from("-1"), allow_hyphen_values=true)]
    pub cols: String,
    /// keep first or last
    #[arg(long, default_value_t = false)]
    pub keep_last: bool,
    /// Get the nth worksheet of EXCEL file
    #[arg(short = 'S', long, default_value_t = 0)]
    pub sheet: usize,
    /// Export to a file named drop-duplicates.csv?
    #[arg(short = 'E', long, default_value_t = false)]
    pub export: bool,
}

macro_rules! impl_row_split {
    ($cmd:ident) => {
        impl $cmd {
            #[allow(dead_code)]
            pub fn split<'a>(&self, row: &'a str) -> CsvRowSplitter<'a> {
                CsvRowSplitter::new(row, self.sep, self.quote)
            }

            #[allow(dead_code)]
            pub fn split_row_to_vec<'a>(&self, row: &'a str) -> Vec<&'a str> {
                CsvRowSplitter::new(row, self.sep, self.quote).collect()
            }

            #[allow(dead_code)]
            pub fn split_row_to_owned_vec<'a>(&self, row: &'a str) -> Vec<String> {
                CsvRowSplitter::new(row, self.sep, self.quote)
                    .map(|i| i.to_owned())
                    .collect::<Vec<_>>()
            }

            #[allow(dead_code)]
            pub fn row_field_count<'a>(&self, row: &'a str) -> usize {
                CsvRowSplitter::new(row, self.sep, self.quote).count()
            }
        }
    };
}

impl_row_split!(Head);
impl_row_split!(Headers);
impl_row_split!(Frequency);
impl_row_split!(Select);
impl_row_split!(Stats);
impl_row_split!(To);
impl_row_split!(Flatten);
impl_row_split!(Unique);
impl_row_split!(Search);
impl_row_split!(Table);
impl_row_split!(Split);
impl_row_split!(Excel2csv);
