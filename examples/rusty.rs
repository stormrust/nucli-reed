use nu_cli_rusty::{create_default_context, Options};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::new();
    let context = create_default_context(true)?;
    nu_cli_rusty::cli(context, options)?;
    Ok(())
}