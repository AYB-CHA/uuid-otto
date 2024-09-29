use uuid::otto::Manager;

mod uuid;

fn main() {
    let a = Manager::new();

    println!("{}", a.get_nill());
}
