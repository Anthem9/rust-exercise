mod module1;
mod module2 {
    pub mod submodule;
}

fn main() {
    module1::print_a_to_z();
    module2::submodule::print_a_to_z();
}
