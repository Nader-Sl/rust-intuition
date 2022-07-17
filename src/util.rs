pub fn print_type_of<T>(str: &str, _: &T) {
    println!("{} {}", str, std::any::type_name::<T>())
}
