mod background;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    background::run()?;
    Ok(())
}
