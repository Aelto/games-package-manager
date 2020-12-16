use std::result::Result;
use games_package_manager::project::{ProjectInformation, Project};
use crate::prompt::prompt_with_question;

pub fn init(skip_prompts: bool) -> Result<(), &'static str> {
  let author_name = "your-name".to_owned();
  let package_name = "package-name".to_owned();
  let package_version = "v0.1".to_owned();
  let package_displayed_name = "displayed-package-name".to_owned();
  let package_description = "your package description".to_owned();
  let license = "your package license".to_owned();

  let project_information = if skip_prompts {
    ProjectInformation::new(
      &author_name,
      &package_name,
      &package_version,
      &package_displayed_name,
      &package_description,
      &license
    )
  }
  else {
    ProjectInformation::new(
      &prompt_with_question("Your name").unwrap_or(author_name),
      &prompt_with_question("Package name").unwrap_or(package_name),
      &prompt_with_question("Package version").unwrap_or(package_version),
      &prompt_with_question("Displayed package name").unwrap_or(package_displayed_name),
      &prompt_with_question("Package description").unwrap_or(package_description),
      &prompt_with_question("Package license").unwrap_or(license),
    )
  };

  Ok(())
}