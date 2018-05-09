pub fn get_tokens(code: String) -> Vec<String> {
    code.replace("(", "( ")
        .replace(")", " )")
        .split_whitespace()
        .map(|t| t.to_owned())
        .collect()
}

#[derive(Debug, Clone)]
pub struct SExpression {
    value: String,
    list: Option<Vec<Box<SExpression>>>,
    parent: Option<Box<SExpression>>
}

impl SExpression {
    fn get_value(&self) -> String {
        self.value.to_owned()
    }
    fn get_parent(&self) -> Option<SExpression> {
        match self.parent {
            Some(ref parent) => {
                let parent = &**parent;
                Some(parent.clone())
            },
            None => None
        }
    }
    fn get_list(&self) -> Option<Vec<SExpression>> {
        match self.list {
            Some(ref list) => {
                let mut vec = Vec::new();
                for item in list {
                    let item = &**item;
                    vec.push(item.clone());
                }
                Some(vec)
            },
            None => None
        }
    }
}
