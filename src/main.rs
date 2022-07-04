use ego_tree::NodeRef;
use itertools::Itertools;
use scraper::{ElementRef, Html, Node, Selector};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    url: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    eprintln!("Fetching results from: {}", opts.url);

    let body: String = ureq::get(&&opts.url).call()?.into_string()?;

    eprintln!("Parsing HTML");

    let html = Html::parse_document(&body);

    let header_selector = Selector::parse("table[id='data-table'] thead tr th").unwrap();
    let mut headers: Vec<String> = Vec::new();
    for header_node in html.select(&header_selector) {
        headers.push(header_node.text().collect());
    }
    eprintln!("Found headers: {}", headers.join(", "));

    // chelou la structure
    let row_selector = Selector::parse("table[id='data-table'] tr").unwrap();

    let mut data: Vec<Vec<String>> = Vec::new();
    for row_node in html.select(&row_selector) {
        let mut row = Vec::new();
        for child in row_node.children() {
            if child.value().is_element() && child.value().as_element().unwrap().name() == "td" {
                let text = extract_text(child.children()).trim().to_string();
                row.push(text);
            }
        }
        if row.len() > 0 {
            eprintln!("Results: {}", row.join(", "));
            data.push(row);
        }
    }
    eprintln!("Found {} results", data.len());

    print_csv_line(&headers);
    for result in data {
        print_csv_line(&result);
    }

    Ok(())
}

fn print_csv_line(line: &[String]) {
    println!(
        "{}",
        line.iter()
            .map(|v| format!("\"{}\"", v.replace("\"", "\"\"")))
            .join(",")
    );
}

fn extract_text<'a, I>(node_iterator: I) -> String
where
    I: Iterator<Item = NodeRef<'a, Node>>,
{
    node_iterator
        .filter(|e| {
            // only keep text nodes, where parent are not <script> nor <style> tags
            e.value().is_text()
                && e.parent()
                    .map(ElementRef::wrap)
                    .flatten()
                    .filter(|e| e.value().name() != "script" && e.value().name() != "style")
                    .is_some()
        })
        .map(|e| e.value().as_text().unwrap().to_string())
        .collect::<String>()
        .split_whitespace()
        .join(" ")
}
