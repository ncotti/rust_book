use textwrap;

fn main() {
    let text = "textwrap: an efficient and powerful library for wrapping text.";
    let text = textwrap::wrap(text, 28);

    for line in text {
        println!("{}", line);
    }
}

fn create_table() {
    
}
