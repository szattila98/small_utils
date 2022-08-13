use chrono::NaiveDate;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "tempestas")]
/// A weather forecast CLI
pub enum Args {
    /// Weather summary of today
    Summary {
        /// Plus days to forecast in addition
        #[structopt(short, long, conflicts_with_all = &["start-date","end-date"])]
        day_no: Option<u8>,
        /// Start date to show weather data for
        #[structopt(short, long, requires = "end-date")]
        start_date: Option<NaiveDate>,
        /// End date to show weather data for
        #[structopt(short, long, requires = "start-date")]
        end_date: Option<NaiveDate>,
    },
    /// Detailed weather forecast for today
    Detailed {
        /// Detailed weather forecast for a specific date
        #[structopt(short, long)]
        specific_date: Option<NaiveDate>,
    },
}
