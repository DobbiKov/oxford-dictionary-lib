use loggit::{self, debug, trace};
use reqwest;
use scraper::{html::Select, ElementRef, Html, Selector};

pub async fn search_dictionary(
    word: &str,
) -> Result<ParseLinkResult, Box<dyn std::error::Error + Send + Sync>> {
    let link = format!(
        "https://www.oxfordlearnersdictionaries.com/search/english/?q={}",
        word
    );

    parse_link(link.as_str()).await
}

pub enum ParseLinkResult {
    ResultList(Vec<String>),
    MeaningsList(Vec<String>),
    None,
}

pub async fn parse_link(
    link: &str,
) -> Result<ParseLinkResult, Box<dyn std::error::Error + Send + Sync>> {
    let body = reqwest::get(link).await?.text().await?;
    let document = Html::parse_document(&body);

    let result_list_res = parse_result_list_by_document(document.clone()).await?;
    let meanings_list_res = parse_meanings_by_document(document).await?;

    if let Some(meaning_res) = meanings_list_res {
        return Ok(ParseLinkResult::MeaningsList(meaning_res));
    }
    if let Some(result_res) = result_list_res {
        return Ok(ParseLinkResult::ResultList(result_res));
    }
    return Ok(ParseLinkResult::None);
}
//class to use for results: result-list
async fn parse_result_list_by_document(
    document: Html,
) -> Result<Option<Vec<String>>, Box<dyn std::error::Error + Send + Sync>> {
    let result_list_selector = Selector::parse("ul.result-list").unwrap();
    let mut ul_res = document.select(&result_list_selector);
    if let Some(ul_elem) = ul_res.next() {
        let mut res: Vec<String> = Vec::new();
        let ul_html = Html::parse_fragment(&ul_elem.html());

        trace!("{}", ul_html.html());
        trace!("");
        let word_selector = Selector::parse("li > a.dym-link").unwrap();

        let words_elems = ul_html.select(&word_selector);
        for word_elem in words_elems {
            trace!("{:?}", word_elem);
            res.push(word_elem.text().next().unwrap_or("").to_string())
        }
        Ok(Some(res))
    } else {
        Ok(None)
    }
}
async fn parse_meanings_by_document(
    document: Html,
) -> Result<Option<Vec<String>>, Box<dyn std::error::Error + Send + Sync>> {
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
