use dom_content_extraction::scraper::Html;
use dom_content_extraction::{get_node_text, scraper, DensityTree};
use std::collections::HashMap;
use worker::*;

// extract query parameters from the URL
fn get_query_params(url: Url) -> Result<HashMap<String, String>> {
    Ok(url.query_pairs().into_owned().collect())
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();
    let url = req.url().unwrap();
    let params = get_query_params(url)?;

    let target_url = params.get("url").ok_or("error")?;

    let response = reqwest::get(target_url).await;

    let body = match response {
        Ok(val) => val.text().await.expect("fail to get body"),
        Err(_) => return Response::error("fail to get body", 500),
    };

    let document = Html::parse_document(&body);

    let mut dtree = DensityTree::from_document(&document).unwrap();

    let _ = dtree.calculate_density_sum();
    let extracted_content = dtree.extract_content(&document).unwrap();

    Response::ok(extracted_content)
}
