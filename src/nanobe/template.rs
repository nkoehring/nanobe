#[derive(BartDisplay)]
#[template = "src/templates/test_child.html"]
struct Section<'a> {
    title: &'a str,
    paragraphs: &'a Vec<&'a str>,
}

#[derive(BartDisplay)]
#[template = "src/templates/test.html"]
struct Article<'a> {
    title: &'a str,
    sections: &'a Vec<Section<'a>>,
}

pub fn test() {
    let paragraphs_one = vec!["section one paragraph one", "section one paragraph two"];
    let paragraphs_two = vec!["section two paragraph one", "section two paragraph two"];
    let sections = vec![
      Section { title: "Section One", paragraphs: &paragraphs_one },
      Section { title: "Section Two", paragraphs: &paragraphs_two },
    ];

    print!("{}", &Article { title: "bart template test", sections: &sections });
}
