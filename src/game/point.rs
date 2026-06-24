struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn get_block() -> &'static str {
        "\u{001b}[0;32m█\u{001b}[0m"
    }
}
