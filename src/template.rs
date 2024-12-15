use crate::judger::JudgerCode;

pub struct Template {
    text: String,
    code: JudgerCode,
}

impl Template {
    pub fn new(text: String, code: JudgerCode) -> Template {
        Template { text, code }
    }
    pub fn render(&self) -> String {
        include_str!("../html/result.html")
            .replace("{status}", &*self.code.as_str_name().to_uppercase())
            .replace("{text}", &*self.text)
    }
}
