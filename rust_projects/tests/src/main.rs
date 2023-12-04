use textwrap;

fn main() {
    let text = "textwrap: an efficient and powerful library for wrapping text.";

    let output = textwrap::wrap(text, 28);

    for element in output {
        println!("{}", element);
    }

    println!("{}", textwrap::termwidth());
     
}
