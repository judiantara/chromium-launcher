use std::fs;
use std::process::{ Command, Stdio };
use clap::Parser;
use directories::ProjectDirs;

#[derive(Parser)]
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

  fs::create_dir_all(&profile_path).expect("Failed to create profile base directory");
  println!("profile path: {}", profile_path.display());

  Command::new("chromium")
    .arg(format!("--user-data-dir={}", profile_path.to_str().expect("path not found")))
    .arg(format!("--class={}", &args.name))
    .arg(format!("--name={}", &args.name))
    .arg(format!("--window-name={}", &args.name))
    .arg("--new-window")
    .arg("--bwsi")
    .arg("--no-default-browser-check")
    .arg(format!("--app={}", &args.url))
    .stderr(Stdio::null())
    .spawn()
    .expect("Failed to run chromium")
    .wait()
    .expect(format!("{} didn't start", &args.name).as_str());
}
