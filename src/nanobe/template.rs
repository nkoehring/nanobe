#[derive(BartDisplay)]
#[template = "src/templates/test.html"]
struct Template<'a> {
    lang: &'a str,
    title: &'a str,
    name: &'a str,
}

pub fn test() {
    print!("{}", &Template { lang: "en", title: "bart template test", name: "Bart" });
}
