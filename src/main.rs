use predict_pass_api::generator::password::generate_password;

fn main() {
    println!("Hello, world!");
    let pass = generate_password(String::from("Batman123"), String::from("uuid-man"));

    println!("{}", pass);
}
