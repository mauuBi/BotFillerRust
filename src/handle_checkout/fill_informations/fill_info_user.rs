
const SCRIPT: &str = r#"
		const data = {
			"email": "mohamednassere01@gmail.com",
			"firstName": "Mohamed",
			"lastName": "Nassere",
			"address1": "4 rue formagne",
			"postalCode": "93500",
			"city": "Pantin",
			"phone": "0695613353"
		};
		
		for (const [name, value] of Object.entries(data)) {
			const input = document.querySelector(`input[name="${name}"]`);
			if (input) {
				input.value = value;
				// On force le site à détecter que la case est remplie
				input.dispatchEvent(new Event('input', { bubbles: true }));
				input.dispatchEvent(new Event('change', { bubbles: true }));
			}
		}
	"#;

pub async fn handle_checkout_turbo(page: &playwright::api::Page) {
    println!("Je suis dedans");
	let _ = page.wait_for_selector_builder("input[name='email']")
		.wait_for_selector()
		.await;
	println!("J'ai trouve le mail");
    // let _ = page.fill_builder("input[name='email']", "").fill().await;

	let _ = page.eval::<()>(SCRIPT).await;
    println!("Remplissage fini !");
}

pub async fn handle_checkout(page: &playwright::api::Page) -> Result<(), Box<dyn std::error::Error>> {
    // // Utilisation correcte du Builder pour attendre


    println!("Mail trouvé");
	loop {
    	match page.fill_builder("input[name='email']", "mohamednassere01@gmail.com").fill().await{
        	Ok(_) => {
        	    println!("Email rempli avec succès !");
        	    break;
        	},
        	Err(_) => {
        	    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        	    println!("... En attente du champ email ...");
        	}
    	}
	}
    let _ = page.fill_builder("input[name='firstName']", "Mohamed").fill();
    let _ = page.fill_builder("input[name='lastName']", "Nassere").fill();
    let _ = page.fill_builder("input[name='address1']", "4 rue formagne").fill();
    let _ = page.fill_builder("input[name='postalCode']", "93500").fill();
    let _ = page.fill_builder("input[name='city']", "Pantin").fill();
    let _ = page.fill_builder("input[name='phone']", "0695613353").fill();

    println!("Remplissage fini !");
    Ok(())
}



pub async fn add_to_cart(page: &playwright::api::Page) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 En attente du bouton Add to Cart...");
    let selector = "button[class='shopify-payment-button__button shopify-payment-button__button--unbranded']"; // On gère les deux cas possibles
	println!("Bouton trouve");
    // 2. On clique instantanément
    let _ = page.click_builder(selector).click();
    
    println!("Produit ajouté au panier !");
    Ok(())
}

