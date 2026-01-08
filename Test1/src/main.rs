// Static

mod static_mod {
    // Public instance of mod
    pub mod api {
        // Public const function
        pub const fn test() -> i32 {
            return 5;
        }
    }
}

// Fuse said mod public function to a usable/callable state.
use crate::static_mod::api::test;

pub(crate) fn main() {
    const RESULT: i32 = test();
    println!("{RESULT}");
    return;
}
