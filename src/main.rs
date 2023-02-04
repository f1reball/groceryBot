

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get("https://www.woolworths.com.au/apis/ui/PiesCategoriesWithSpecials")?
        .text()?;
    println!("{:#?}", resp);
    Ok(())
}