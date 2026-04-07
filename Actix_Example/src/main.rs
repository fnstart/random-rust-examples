mod modules;

pub(crate) mod static_mod {}

fn main() {
    const RESULT: f64 = modules::base::api::value_of_pi();
    println!("{RESULT}");

    return;
}
