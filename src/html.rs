use std::fmt::{Display, Formatter, Result};

pub enum Charset {
    Utf8,
}

pub enum Lang {
    En,
}

pub enum HtmlDom {
    Html(Lang, Vec<HtmlDom>),
    Head(Vec<HtmlDom>),
    Meta(Option<Charset>, Vec<HtmlDom>),
    Body(Vec<HtmlDom>),
    Title(Vec<HtmlDom>),
    H1(Vec<HtmlDom>),
    P(Vec<HtmlDom>),
    Text(String),
}

pub struct Html {
    pub root_node: HtmlDom,
}

impl Display for Charset {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match self {
            Charset::Utf8 => "utf-8",
        })
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", match self {
            Lang::En => "en",
        })
    }
}

impl Display for HtmlDom {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            HtmlDom::Html(lang, elts) => {
                writeln!(f, "<html lang=\"{}\">", lang.to_string())?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</html>")
            },
            HtmlDom::Head(elts) => {
                writeln!(f, "<head>")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</head>")
            },
            HtmlDom::Meta(charset, elts) => {
                write!(f, "<meta")?;
                match charset {
                    Some(set) => {
                        write!(f, " charset=\"{}\"", set)?;
                    },
                    None => (),
                };
                writeln!(f, ">")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</meta>")
            },
            HtmlDom::Body(elts) => {
                writeln!(f, "<body>")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</body>")
            },
            HtmlDom::Title(elts) => {
                writeln!(f, "<title>")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</title>")
            },
            HtmlDom::H1(elts) => {
                writeln!(f, "<h1>")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</h1>")
            },
            HtmlDom::P(elts) => {
                writeln!(f, "<p>")?;
                for elt in elts.iter() {
                    elt.fmt(f)?
                }
                writeln!(f, "</p>")
            },
            HtmlDom::Text(elt) => {
                writeln!(f, "{}", elt)
            }
        }
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(f, "<!DOCTYPE html>")?;
        self.root_node.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html_formatted_properly() {
        let html = Html {
            root_node: HtmlDom::Html(Lang::En, vec![
                HtmlDom::Head(vec![
                    HtmlDom::Meta(Some(Charset::Utf8), vec![]),
                    HtmlDom::Title(vec![
                        HtmlDom::Text(String::from("Hello!"))])]),
                HtmlDom::Body(vec![
                    HtmlDom::H1(vec![
                        HtmlDom::Text(String::from("Hello!"))]),
                    HtmlDom::P(vec![
                        HtmlDom::Text(String::from("Hi from Rust!"))])])])};
        assert_eq!("\
<!DOCTYPE html>
<html lang=\"en\">
<head>
<meta charset=\"utf-8\">
</meta>
<title>
Hello!
</title>
</head>
<body>
<h1>
Hello!
</h1>
<p>
Hi from Rust!
</p>
</body>
</html>
",
        html.to_string());
    }
}
