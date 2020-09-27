use crate::models::datarow::{Data, DataType};
use std::ops::Add;

pub struct Bitbar {}

impl Bitbar {
    fn transform_data_type(data_row: &Data) -> String {
        let task_in_progress_icon = "small_orange_diamond";
        let task_to_do_icon = "small_red_triangle";
        let task_done_icon = "small_blue_diamond";

        let indent = if data_row.indent == 0 { "".to_string() } else { "--".repeat(data_row.indent).add(" ")};

        match data_row.data_type {
            DataType::Title => format!("---\n{}\n---", data_row.data),
            DataType::SubTitle => format!("---\n{}\n---", data_row.data),
            DataType::TaskToDo => format!("{}:{}: {} | color={}", &indent, task_to_do_icon, data_row.data, "red"),
            DataType::TaskInProgress => format!("{}:{}: {} | color={}", &indent, task_in_progress_icon, data_row.data, "yellow"),
            DataType::TaskDone => format!("{}:{}: {} | color={}", &indent, task_done_icon, data_row.data, "green"),
            DataType::TaskNote => format!("{}{}", &indent, data_row.data),
        }
    }

    fn transform_data_links(data_row: &Data) -> Vec<String> {
        data_row.links.iter()
            .map(|link|
                format!("{} link: {} | href={} color={}",
                        "--".repeat(data_row.indent + 1),
                        link.name,
                        link.url,
                        "blue")
            )
            .collect()
    }

    pub fn process(data_rows: Vec::<Data>, count: usize, editor: &str, source_file: &str, icon: &str, empty_icon: &str) {
        println!(":{}:{}\n---", if count == 0 && !empty_icon.is_empty() { empty_icon } else { icon }, count);
        for data_row in data_rows {
            println!("{}", Bitbar::transform_data_type(&data_row));
            Bitbar::transform_data_links(&data_row).iter().for_each(|row| println!("{}", row));
        }
        if !editor.is_empty() {
            println!("---\nOpen file | size=-4 bash={} param1={} terminal=false\n---", editor, source_file);
        }
    }
}

#[test]
fn task_todo_data_row_should_output_correctly() {
    use crate::models::datarow::DataRowProcessor;

    let line = "  * [ ] something";
    let data = DataRowProcessor::from(line);
    let row = data.first().unwrap();
    let result = Bitbar::transform_data_type(row);
    assert_eq!("-- :small_red_triangle: something", result);
}

#[test]
fn task_done_data_row_should_output_correctly() {
    use crate::models::datarow::DataRowProcessor;

    let line = "  * [x] something";
    let data = DataRowProcessor::from(line);
    let row = data.first().unwrap();
    let result = Bitbar::transform_data_type(row);
    assert_eq!("-- :small_blue_diamond: something", result);
}
