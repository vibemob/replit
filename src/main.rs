
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct StockQuote {
    symbol: String,
    name: String,
    price: f64,
    #[serde(rename = "previousClose")]
    previous_close: f64,
    open: f64,
    #[serde(rename = "earningsDate")]
    earnings_date: Option<String>,
    eps: Option<f64>,
    pe: Option<f64>,
    beta: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
struct SearchResult {
    symbol: String,
    name: String,
}

const API_KEY: &str = "YOUR_API_KEY_HERE"; // Replace with your actual API key

fn main() {
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let search_query = use_state(cx, || String::new());
    let search_results = use_state(cx, || Vec::<SearchResult>::new());
    let selected_stock = use_state(cx, || Option::<StockQuote>::None);
    let loading = use_state(cx, || false);
    let error_message = use_state(cx, || Option::<String>::None);

    let search_stocks = move |_| {
        let query = search_query.get().clone();
        if query.is_empty() {
            return;
        }

        loading.set(true);
        error_message.set(None);
        
        let search_results = search_results.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        cx.spawn(async move {
            match fetch_search_results(&query).await {
                Ok(results) => {
                    search_results.set(results);
                    loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Search failed: {}", e)));
                    loading.set(false);
                }
            }
        });
    };

    let get_stock_details = move |symbol: String| {
        loading.set(true);
        error_message.set(None);
        
        let selected_stock = selected_stock.clone();
        let loading = loading.clone();
        let error_message = error_message.clone();

        cx.spawn(async move {
            match fetch_stock_quote(&symbol).await {
                Ok(quote) => {
                    selected_stock.set(Some(quote));
                    loading.set(false);
                }
                Err(e) => {
                    error_message.set(Some(format!("Failed to fetch stock details: {}", e)));
                    loading.set(false);
                }
            }
        });
    };

    render! {
        div {
            style: "font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px;",
            
            h1 { "Stock Search App" }
            
            div {
                style: "margin-bottom: 20px;",
                input {
                    r#type: "text",
                    placeholder: "Search for a company (e.g., Apple, Microsoft)",
                    value: "{search_query}",
                    style: "padding: 10px; width: 300px; margin-right: 10px; border: 1px solid #ccc; border-radius: 4px;",
                    oninput: move |evt| search_query.set(evt.value.clone()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            search_stocks(());
                        }
                    }
                }
                button {
                    onclick: search_stocks,
                    style: "padding: 10px 20px; background-color: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;",
                    disabled: *loading.get(),
                    if *loading.get() { "Searching..." } else { "Search" }
                }
            }

            if let Some(error) = error_message.get() {
                div {
                    style: "color: red; margin-bottom: 20px; padding: 10px; background-color: #ffe6e6; border-radius: 4px;",
                    "{error}"
                }
            }

            if !search_results.is_empty() {
                div {
                    style: "margin-bottom: 20px;",
                    h3 { "Search Results:" }
                    for result in search_results.iter() {
                        div {
                            key: "{result.symbol}",
                            style: "padding: 10px; margin: 5px 0; border: 1px solid #ddd; border-radius: 4px; cursor: pointer; background-color: #f9f9f9;",
                            onclick: move |_| get_stock_details(result.symbol.clone()),
                            "{result.name} ({result.symbol})"
                        }
                    }
                }
            }

            if let Some(stock) = selected_stock.get() {
                div {
                    style: "border: 2px solid #007bff; border-radius: 8px; padding: 20px; background-color: #f8f9fa;",
                    h2 { "{stock.name} ({stock.symbol})" }
                    
                    div {
                        style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 15px; margin-top: 15px;",
                        
                        div {
                            style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                            strong { "Current Price:" }
                            div { style: "font-size: 1.2em; color: #28a745;", "${stock.price:.2}" }
                        }
                        
                        div {
                            style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                            strong { "Previous Close:" }
                            div { "${stock.previous_close:.2}" }
                        }
                        
                        div {
                            style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                            strong { "Opening Price:" }
                            div { "${stock.open:.2}" }
                        }
                        
                        if let Some(earnings_date) = &stock.earnings_date {
                            div {
                                style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                strong { "Earnings Date:" }
                                div { "{earnings_date}" }
                            }
                        }
                        
                        if let Some(eps) = stock.eps {
                            div {
                                style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                strong { "EPS:" }
                                div { "{eps:.2}" }
                            }
                        }
                        
                        if let Some(pe) = stock.pe {
                            div {
                                style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                strong { "PE Ratio:" }
                                div { "{pe:.2}" }
                            }
                        }
                        
                        if let Some(beta) = stock.beta {
                            div {
                                style: "padding: 10px; background-color: white; border-radius: 4px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                                strong { "Beta:" }
                                div { "{beta:.2}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn fetch_search_results(query: &str) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/search?query={}&limit=10&apikey={}",
        query, API_KEY
    );
    
    let response = reqwest::get(&url).await?;
    let results: Vec<SearchResult> = response.json().await?;
    Ok(results)
}

async fn fetch_stock_quote(symbol: &str) -> Result<StockQuote, Box<dyn std::error::Error>> {
    // First get the quote data
    let quote_url = format!(
        "https://financialmodelingprep.com/api/v3/quote/{}?apikey={}",
        symbol, API_KEY
    );
    
    let response = reqwest::get(&quote_url).await?;
    let quotes: Vec<serde_json::Value> = response.json().await?;
    
    if quotes.is_empty() {
        return Err("No quote data found".into());
    }
    
    let quote_data = &quotes[0];
    
    // Get company profile for additional data
    let profile_url = format!(
        "https://financialmodelingprep.com/api/v3/profile/{}?apikey={}",
        symbol, API_KEY
    );
    
    let profile_response = reqwest::get(&profile_url).await?;
    let profiles: Vec<serde_json::Value> = profile_response.json().await?;
    
    let profile_data = profiles.get(0);
    
    let stock_quote = StockQuote {
        symbol: quote_data["symbol"].as_str().unwrap_or("").to_string(),
        name: quote_data["name"].as_str().unwrap_or("").to_string(),
        price: quote_data["price"].as_f64().unwrap_or(0.0),
        previous_close: quote_data["previousClose"].as_f64().unwrap_or(0.0),
        open: quote_data["open"].as_f64().unwrap_or(0.0),
        earnings_date: quote_data["earningsAnnouncement"]
            .as_str()
            .map(|s| s.to_string()),
        eps: quote_data["eps"].as_f64(),
        pe: quote_data["pe"].as_f64(),
        beta: profile_data
            .and_then(|p| p["beta"].as_f64()),
    };
    
    Ok(stock_quote)
}
