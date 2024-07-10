use std::path::PathBuf;

use crate::error::Res;

use super::{context::Context, TEMPLATE_DIR};

pub trait TemplateStrategy {
    fn get_dir_name(&self) -> &str;
    fn list_templates(&self) -> Res<Vec<String>>;
    fn render_template(&self, template: &str, context: &Context) -> Res<()>;
    fn render_templates(&self, context: &Context) -> Res<()> {
        for template in self.list_templates()? {
            self.render_template(&template, context)?;
        }

        Ok(())
    }
}

pub enum ProjectType {
    Single,
    Multiple,
    Modpack,
}

impl ProjectType {
    pub fn strategy<'a>(self, target: PathBuf) -> ProjectTemplateStrategy<'a> {
        let typ = match self {
            ProjectType::Single => "single",
            ProjectType::Multiple => "multiple",
            ProjectType::Modpack => "modpack",
        };
        ProjectTemplateStrategy::new(typ, target)
    }
}

pub struct ProjectTemplateStrategy<'a> {
    env: minijinja::Environment<'a>,
    typ: String,
    target: PathBuf,
}

impl<'a> ProjectTemplateStrategy<'a> {
    fn new(typ: &str, target: PathBuf) -> Self {
        let mut env = minijinja::Environment::new();
        env.set_loader(minijinja::path_loader(TEMPLATE_DIR));

        Self {
            env,
            typ: typ.to_string(),
            target,
        }
    }

    pub fn get_template(&self, template: &str) -> minijinja::Template {
        self.env.get_template(template).unwrap()
    }
}

impl<'a> TemplateStrategy for ProjectTemplateStrategy<'a> {
    fn get_dir_name(&self) -> &str {
        self.typ.as_str()
    }

    fn list_templates(&self) -> Res<Vec<String>> {
        let dir = PathBuf::from(TEMPLATE_DIR).join(&self.get_dir_name());

        Ok(dir
            .read_dir()?
            .into_iter()
            .map(|entry| {
                entry.map(|e| {
                    e.path()
                        .to_string_lossy()
                        .to_string()
                        .strip_prefix(&format!("{}/", TEMPLATE_DIR))
                        .unwrap()
                        .to_string()
                })
            })
            .collect::<Result<Vec<String>, std::io::Error>>()?
            .into_iter()
            .filter(|entry| !entry.ends_with(".gitignore"))
            .collect())
    }

    fn render_template(&self, template: &str, context: &Context) -> Res<()> {
        let filename = template
            .strip_prefix(&format!("{}/", self.get_dir_name()))
            .unwrap();

        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(self.target.join(filename))?;

        let template = self.get_template(template);
        template.render_to_write(context, file)?;

        Ok(())
    }

    fn render_templates(&self, context: &Context) -> Res<()> {
        if self.typ == "common" {
            for template in self.list_templates()? {
                self.render_template(&template, context)?;
            }

            return Ok(());
        }

        let common_strategy = ProjectTemplateStrategy::new("common", self.target.clone());
        common_strategy.render_templates(context)
    }
}

#[cfg(test)]
mod tests {
    use crate::template::context::Context;

    use super::*;

    #[test]
    fn test_single_mod_strategy() {
        let strat = ProjectType::Single.strategy(PathBuf::from("tmp"));
        let context = Context::new("mod", "My Mod", "0.0.1", "Author");

        strat
            .render_templates(&context)
            .expect("Failed to render templates");
    }
}
