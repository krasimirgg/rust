//@ only-x86_64
#[target_feature(enable = "movrs")]
//~^ ERROR: currently unstable
unsafe fn foo() {}

fn main() {}
