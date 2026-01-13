use playwright::{Playwright};
use std::{env::args, path::Path};

mod handle_checkout;

use handle_checkout::fill_informations::fill_info_user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::initialize().await?;

    let chromium = playwright.chromium();
    
    let browser = chromium.launcher()
        .args(&vec![
            "--disable-background-timer-throttling".to_string(),
            "--disable-dev-shm-usage".to_string(),
            "--disable-gpu".to_string(),
            "--no-sandbox".to_string(),
        ])
        .executable(Path::new("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"))
        .headless(false)
        .launch()
        .await?;
    let context = browser.context_builder().build().await?;
    context.add_init_script(r#"
        const style = document.createElement('style');
        style.textContent = `* { transition: none !important; animation: none !important; }`;
        document.head.appendChild(style);
    "#).await?;
    let page = context.new_page().await?;
    let url_to_go = String::from("https://gainzclub.co/collections/all");
    let new_page = page.clone();
    
    tokio::spawn(async move {
        let _ = page.goto_builder(&url_to_go).goto().await;
    });
    let mut already_carted = false;
    loop {
        let url: String = new_page.url()?;
        if check_checkout(&url){
            println!("Page de paiement");
            // fill_info_user::handle_checkout(&new_page).await?;
            fill_info_user::handle_checkout_turbo(&new_page).await;
            break;
        } else if check_cart(&url) && !already_carted {
            println!("Page de cart");
            fill_info_user::add_to_cart(&new_page).await?;
            already_carted = true;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    loop {
        
    }
    return Ok(());
}

fn check_checkout(url: &str) -> bool{
    url.contains("checkout")
}

fn check_cart(url: &str) -> bool{
    url.contains("product")
}
