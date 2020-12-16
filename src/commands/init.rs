use std::result::Result;

pub fn init(skip_prompts: bool) -> Result<(), &'static str> {
  println!("init command called, skip_promts: {}", skip_prompts);

  Ok(())
}