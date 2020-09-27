trait LineCategory {}
struct Line {
    data: String,
    indentation: usize
}

trait Heading {
    type LineCategory;
    fn process(&self) -> Option<String>;
    fn new(line: String) -> Self;
}

impl Heading for Line {
    type LineCategory = Line;
    fn new(line: String) -> Self {
        Line {
            data: format!("---\n{}\n---\n", line.split("# ").last().unwrap().trim()),
            indentation: 0,
        }
    }

    fn process(&self) -> Option<String> {
        None
    }
}



