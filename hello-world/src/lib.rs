pub fn hello(name: Option<&str>) -> String {
    /*
    match name {
        Some(inner) => format!("Hello, {}!", inner),
        None        => format!("Hello, World!")
    }
    */
    let mut output = "World";
    if let Some(c) = name {
        output = c;
    };
    format!("Hello, {}!", output)
}
