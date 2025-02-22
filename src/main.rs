use anyhow::Result;

use rustlox::Lox;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);

        std::process::exit(64);
    } else if args.len() == 2 {
        Lox::new().run_file(&args[1]).unwrap();
    } else {
        Lox::new().run_prompt().unwrap();
    }

    Ok(())
}
