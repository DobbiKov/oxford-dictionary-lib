use loggit::{self, debug, trace};
use reqwest::Client;
use scraper::{html::Select, ElementRef, Html, Selector};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    loggit::logger::set_colorized(true);
    loggit::logger::set_global_formatting("{line}|<red>{level}<red>: {message}");
    loggit::logger::set_log_level(loggit::Level::TRACE);

    let res = search_dictionary("inconvinence").await;
    if let Err(e) = res {
        println!("pizda, error nahuy");
    } else {
        let res_o = res.unwrap();
        if res_o.is_none() {
            println!("Net slova!");
        } else {
            println!("{:?}", res_o.unwrap());
        }
    }
    Ok(())
}

async fn search_dictionary(word: &str) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
    //let link = format!(
    //    "https://www.oxfordlearnersdictionaries.com/definition/english/{}?q={}",
    //    &word, &word
    //);
    let link = format!(
        "https://www.oxfordlearnersdictionaries.com/search/english/?q={}",
        word
    );
    parse_meanings_by_link(link.as_str()).await
}

//async fn parse_link()
//class to use for results: result-list
async fn parse_meanings_by_link(
    link: &str,
) -> Result<Option<Vec<String>>, Box<dyn std::error::Error>> {
    let body = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&body);
    let meaning_selector = Selector::parse("li.sense").unwrap();
    let meanings_html = document.select(&meaning_selector).collect::<Vec<_>>();

    if meanings_html.is_empty() {
        return Ok(None);
    }
    Ok(Some(get_meanings(meanings_html)))
}

fn get_meanings(meanings_html: Vec<ElementRef<'_>>) -> Vec<String> {
    let mut res = Vec::<String>::new();
    for meaning_html in meanings_html {
        let sense_string = handle_sense_html(Html::parse_fragment(&meaning_html.html()));
        res.push(sense_string);
    }
    res
}

fn handle_sense_html(element: Html) -> String {
    let mut res: String = String::new();
    let label_select = Selector::parse("span.labels").unwrap();
    let def_select = Selector::parse("span.def").unwrap();
    let examples_ul_select = Selector::parse("ul.examples").unwrap();

    if let Some(label) = element.select(&label_select).next() {
        res.push_str(process_label(label).as_str());
    }

    if let Some(def) = element.select(&def_select).next() {
        res.push_str(&def.text().collect::<Vec<&str>>().join(" "));
    }

    res.push_str("\n\n Examples:\n");

    let examples = element.select(&examples_ul_select);
    res.push_str(process_examples_ul(examples).as_str());

    res
}

fn process_label(label: ElementRef<'_>) -> String {
    trace!("{:?}", label);
    let label_first = label
        .text()
        .collect::<Vec<&str>>()
        .first()
        .map_or("", |v| v);

    debug!("label_first: {:?}", label_first);
    label_first.to_string()
}

fn process_examples_ul(element: Select<'_, '_>) -> String {
    let mut res = String::new();
    for example in element {
        res.push_str(&example.text().collect::<Vec<&str>>().join("\n"));
        res.push_str("\n");
    }
    res
}
