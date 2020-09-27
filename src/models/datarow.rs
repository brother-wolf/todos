use regex::Regex;

#[derive(Debug,PartialEq,Clone,Ord,Eq,PartialOrd,Copy)]
pub enum DataType {
    Title,
    SubTitle,
    TaskInProgress,
    TaskToDo,
    TaskNote,
    TaskDone,
}

pub struct Link {
    pub name: String,
    pub url: String,
}


pub struct Data {
    pub data_type: DataType,
    pub data: String,
    pub indent: usize,
    pub links: Vec<Link>
}

impl Data {
    fn count_indents(line: &str) -> usize {
        match line.replace("\t", "  ").find(|c: char| !c.is_whitespace()) {
            Some(count) => count / 2,
                // {
                // let (whitespace, _) = line.split_at(count);
                // println!("whitespace:{}:", whitespace);
                // println!("First:{}:", whitespace.split("\t").next().unwrap());
                // let tabs = whitespace.split("\t").count();
                // let spaces = whitespace.split(" ").count();
                // println!("tabs:{}: spaces:{}:", tabs, spaces);
                // tabs + (spaces/2)
            // },
            None => 0,
        }
    }

    fn new(data_type: DataType, row: &str, indent: usize) -> Vec::<Data> {
        let re = Regex::new(r"\[(?P<name>[^\]]+)\]\((?P<url>[^\)]+)\)").unwrap();
        let links = re.captures_iter(row)
            .map(|cap| Link { name: cap[1].to_string(), url: cap[2].to_string() } )
            .collect::<Vec::<Link>>();
        let linkless_row = re.replace_all(row, "$name").to_string();
        vec![Data { data_type, data: linkless_row, indent, links }]
    }
}

pub struct DataRowProcessor {}


impl DataRowProcessor {
    pub fn from(line: &str) -> Vec<Data> {
        let indent = Data::count_indents(&line);
        let data = line.trim();
        if data.starts_with("# ") { Data::new(DataType::Title, &line.split("# ").last().unwrap().trim(), indent) }
        else if data.starts_with("## ") { Data::new(DataType::SubTitle, &line.split("## ").last().unwrap().trim(), indent) }
        else if data.starts_with("* [ ] !") { Data::new(DataType::TaskInProgress, &line.split("* [ ] ! ").last().unwrap().trim(), indent) }
        else if data.starts_with("* [ ] ") { Data::new(DataType::TaskToDo, &line.split("* [ ] ").last().unwrap().trim(), indent) }
        else if data.starts_with("* [x] ") { Data::new(DataType::TaskDone, &line.split("* [x] ").last().unwrap().trim(), indent) }
        else if data.starts_with("- ") { Data::new(DataType::TaskNote, &line.split("- ").last().unwrap().trim(), indent) }
        else { vec![] }
    }
}

#[test]
fn data_row_processor_should_find_task_in_progress() {
    let result = DataRowProcessor::from("* [ ] ! something something");
    assert!(!result.is_empty());
    let data = result.first().unwrap();
    assert_eq!("something something", data.data);
    assert_eq!(DataType::TaskInProgress, data.data_type);
}

#[test]
fn data_row_processor_should_find_task_done() {
    let result = DataRowProcessor::from("* [x] something something");
    assert!(!result.is_empty());
    let data= result.first().unwrap();
    assert_eq!("something something", data.data);
    assert_eq!(DataType::TaskDone, data.data_type);
}

#[test]
fn data_row_processor_should_find_task_to_do() {
    let result = DataRowProcessor::from("* [ ] something something");
    assert!(!result.is_empty());
    let data = result.first().unwrap();
    assert_eq!("something something", data.data);
    assert_eq!(DataType::TaskToDo, data.data_type);
}


#[test]
fn test_whitespace_count() {
    assert_eq!(1, Data::count_indents("\tfgh"));
}