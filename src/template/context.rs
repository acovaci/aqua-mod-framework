pub const FRAMEWORK_NAME: &str = "Aqua Mod Framework";

#[derive(serde::Serialize)]
pub enum ProjectType {
    Single,
    Multiple,
    Modpack,
}

#[derive(serde::Serialize)]
pub struct Project {
    #[serde(rename = "type")]
    type_: ProjectType,
    name: String,
    title: String,
    version: String,
    author: String,
}

#[derive(serde::Serialize)]
struct Mod {
    name: String,
    title: String,
    version: String,
    author: String,
}

#[derive(serde::Serialize)]
struct Framework {
    name: String,
    version: String,
}

#[derive(serde::Serialize)]
struct Metadata {
    framework: Framework,
    today: String,
}

#[derive(serde::Serialize)]
pub struct Context {
    #[serde(rename = "project")]
    project: Project,
    meta: Metadata,
}

impl Context {
    pub fn new(name: &str, title: &str, version: &str, author: &str) -> Self {
        let project = Project {
            type_: ProjectType::Single,
            name: name.to_string(),
            title: title.to_string(),
            version: version.to_string(),
            author: author.to_string(),
        };

        let framework = Framework {
            name: FRAMEWORK_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        let meta = Metadata {
            framework,
            today: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .to_string(),
        };

        Self { project, meta }
    }
}
