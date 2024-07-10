use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[clap(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    #[clap(about = "Build the Factorio mod project")]
    Build,
    #[clap(about = "Setup the Factorio mod project")]
    Init,
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
}

#[derive(serde::Serialize)]
struct Context {
    #[serde(rename = "mod")]
    mod_: Mod,
    meta: Metadata,
}

fn main() {
    let args = Arguments::parse();

    match args.command {
        Command::Build => build(),
        Command::Init => init(),
        _ => todo!(),
    }
}

fn build() {
    println!("Building...");
}

fn init() {
    // STEPS
    // 1. Init git
    // 2. Compile jinja templates

    println!("Initializing...");

    let output = std::process::Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to init git");

    let mut env = minijinja::Environment::new();
    env.set_loader(minijinja::path_loader("templates"));

    let template = env
        .get_template("single/info.yaml")
        .expect("Failed to load template");

    let modd = Mod {
        name: "mod".to_string(),
        title: "My Mod".to_string(),
        version: "0.0.1".to_string(),
        author: "Author".to_string(),
    };

    let framework = Framework {
        name: "Aqua Mod Framework".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    let context = Context {
        mod_: modd,
        meta: Metadata { framework },
    };

    println!("{}", template.render(context).unwrap());
}
