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
    std::process::Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to init git");
}
