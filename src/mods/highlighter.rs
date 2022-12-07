/* function that takes a single vector and uses highlight_countries */
pub fn _highlight_countries(args: &[String]) {
    if args.len() < 2 {
        eprintln!("Usage: highlight_countries <countries> <selected_country>");
        return;
    }

    let countries = &args[0];
    let selected_country = &args[1];
    println!("{}", highlight_countries(countries, selected_country));
}

/* Function that takes a string of comma seperated countries and highlight the selected one. */
fn highlight_countries(countries: &str, selected_country: &str) -> String {
    let mut highlighted_countries = String::new();
    let countries: Vec<&str> = countries.split(",").collect();

    for country in countries {
        if country == selected_country {
            highlighted_countries.push_str(&format!("-{}- ", country));
        } else {
            highlighted_countries.push_str(&format!("{} ", country));
        }
    }

    highlighted_countries
}

#[test]
fn highlight_countries_test() {
    let countries = "United States,United Kingdom,France,Germany,Italy";
    let selected_country = "France";
    assert_eq!(
        highlight_countries(countries, selected_country),
        "United States United Kingdom -France- Germany Italy "
    );
}