mod create_readme;

use create_readme::create_readme;

fn main() {
    match create_readme() {
        Ok(_v) => println!("README.md file generated correctly"),
        Err(e) => println!("Opps! there was an error: {:?}", e),
    }
}
