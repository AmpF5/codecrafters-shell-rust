pub fn execute(args: &str) {
    println!(
        "{}",
        crate::utils::string::get_formatted_input(args).join(" ")
    );
}
