mod formats;
mod models;

use structopt::StructOpt;
use crate::models::datarow::{Data, DataType, DataRowProcessor};
use crate::formats::format::Format;
use crate::formats::bitbar::Bitbar;

use std::{fs::File, io::{self, BufRead, BufReader}, path::Path};
use std::str::FromStr;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "p", long = "path", help = "path to the input file (needs to be markdown; use '* [ ] ' for tasks, '* [x] ' for completed tasks, '* [ ] ! ' for in progress, '- ' for sub notes, ## for sub-titles, # for title, and indent by two spaces for sub-tasks)", required = true)]
    path: String,
    #[structopt(short = "i", long = "icon", help = "icon to display if there are tasks to do")]
    icon: String,
    #[structopt(short = "n", long = "empty-icon", default_value = "", help = "icon to display if there are no tasks to do; if this is not set then the icon will not be changed if tasks reach zero")]
    empty_icon: String,
    #[structopt(short = "e", long = "editor", default_value = "", help = "path to an editor program that can open the input file")]
    editor: String,
    #[structopt(short = "f", long = "format", default_value = "bitbar", possible_values = &vec![ "bitbar" ], help = "The format for the output; currently only Bitbar is supported, markdown and/or json may be supported at a later date")]
    format: String,
}

fn main() {
    let opt = Opt::from_args();
    let format = Format::from_str(&opt.format);
    let data = lines_from_file(&opt.path).expect("Could not load filename");

    let data_rows: Vec::<Data> = data.iter()
        .flat_map(|row| DataRowProcessor::from(row))
        // .filter(|row| row.is_some())
        // .map(|row| row.unwrap())
        .collect::<Vec::<Data>>();

    let count: usize = data_rows.iter().map(|f|
         match f.data_type {
             DataType::TaskToDo => 1,
             _ => 0
         }).sum();

    match format {
        Ok(Format::Bitbar) => Bitbar::process(data_rows, count, &opt.editor, &opt.path, &opt.icon, &opt.empty_icon),
        Ok(_) => println!("Format not written yet"),
        Err(_) => println!(":warning: invalid format option"),
    }
}
