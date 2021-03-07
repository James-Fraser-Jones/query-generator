use std::fmt;

pub enum Query {
    Add(Vec<String>, Vec<String>),
    Done(u64),
    Search(Vec<WordOrTag>),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Query::Add(words, tags) => {
                let mut description = String::new();
                for word in words {
                    description.push_str(word);
                    description.push(' ');
                }
                description.pop();
                let mut tag_string = String::new();
                for tag in tags {
                    tag_string.push(' ');
                    tag_string.push('#');
                    tag_string.push_str(tag);
                }
                write!(f, "add \"{}\"{}", description, tag_string)
            },
            Query::Done(index) => {
                write!(f, "done {}", index)
            },
            Query::Search(params) => {
                let mut search_string = String::new();
                for param in params {
                    search_string.push(' ');
                    search_string.push_str(&param.to_string());
                }
                write!(f, "search{}", search_string)
            },
        }
    }
}

pub enum WordOrTag {
    Word (String),
    Tag (String),
}

impl fmt::Display for WordOrTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WordOrTag::Word(word) => {
                write!(f, "{}", word)
            },
            WordOrTag::Tag(tag) => {
                write!(f, "#{}", tag)
            },
        }
    }
}