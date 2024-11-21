use std::io::{stdin, Read};

use cli_clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let mut buffer = String::new();
    stdin()
        .read_to_string(&mut buffer)
        .expect("Error reading input");
    copy_to_clipboard(buffer.clone()).expect("Couldn't Copy to Clipboard");
    print!("{buffer}")
}

fn copy_to_clipboard(contents: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(contents)
}
