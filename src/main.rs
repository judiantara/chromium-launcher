use clap::Parser;
use directories::ProjectDirs;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
  /// The name of the web app
  name: String,

  /// The default URL used for the app
  url: String,
}

fn main() {
  let args = Args::parse();

  let project_dir = ProjectDirs::from("opik", "opik", &args.name).expect("Can't construct project directories.");

  let profile_path = project_dir.config_dir();

  std::fs::create_dir_all(&profile_path).expect("Failed to create profile base directory");
  println!("profile path: {}", profile_path.display());

  std::process::Command::new("chromium")
    .arg(format!("--user-data-dir={}", profile_path.to_str().expect("path not found")))
    .arg(format!("--class={}", &args.name))
    .arg(format!("--name={}", &args.name))
    .arg("--new-window")
    .arg("--no-default-browser-check")
    .arg(format!("--app={}", &args.url))
    .spawn()
    .expect("Failed to run chromium")
    .wait()
    .expect(format!("{} exits in error.", &args.name));
}
