use predict_pass_api::generator::password::PasswordGenerator;

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    let pass =
        PasswordGenerator::generate_hash(String::from("Batman123"), String::from("uuid-man"))?;

    println!("{}", pass);

    Ok(())
}
