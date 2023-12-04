use std::fs;

/// The `get_input` function reads the contents of a file at the given path and returns them as a vector of strings, with
/// leading and trailing whitespace removed from each line.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `get_input` function is a `String` that represents the file path from which you
/// want to read the input.
///
/// Returns:
///
/// The function `get_input` returns a `Vec<String>`, which is a vector of strings.
fn get_input(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn main() {
    let path = "res/data.txt";
    let lines = get_input(path);

    for line in lines {
        println!("{}", line);
    }
}
