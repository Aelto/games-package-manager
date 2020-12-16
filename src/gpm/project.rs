use std::result::Result;

pub struct ProjectInformationExtraData {
  key: String,
  value: String
}

impl ProjectInformationExtraData {
  pub fn new(key: &str, value: &str) -> ProjectInformationExtraData {
    ProjectInformationExtraData {
      key: key.to_owned(),
      value: key.to_owned()
    }
  }
}

pub struct ProjectInformation {
  // required values
  creator: String,
  identifier: String,
  version: String,
  display_name: String,
  description: String,
  license: String,

  // optional values from there
  website_url: Option<String>,
  dependencies: Vec<u8>,
  tags: Vec<String>,
  install_strategies: Vec<String>,
  extra_data: Vec<ProjectInformationExtraData>
}

impl ProjectInformation {
  pub fn new (creator: &str, identifier: &str, version: &str, display_name: &str, description: &str, license: &str) -> ProjectInformation {
    ProjectInformation {
      creator: creator.to_owned(),
      identifier: identifier.to_owned(),
      version: version.to_owned(),
      display_name: display_name.to_owned(),
      description: description.to_owned(),
      license: license.to_owned(),

      website_url: None,
      dependencies: Vec::new(),
      tags: Vec::new(),
      install_strategies: Vec::new(),
      extra_data: Vec::new()
    }
  }
}

pub struct Project {
  information: ProjectInformation
}

impl Project {
  pub fn new(project_information: ProjectInformation) -> Project {
    Project {
      information: project_information
    }
  }

  pub fn init_project_directory() -> Result<(), &'static str> {
    Ok(())
  }

  pub fn publish() -> Result<(), &'static str> {
    Ok(())
  }
}