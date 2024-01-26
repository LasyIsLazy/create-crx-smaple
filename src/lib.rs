use std::{fs::create_dir, process::Command};

use directories::ProjectDirs;

pub fn download_sample(update: bool) {
    let repo_link = "git@github.com:GoogleChrome/chrome-extensions-samples.git";
    let repo_name = "chrome-extensions-samples";

    let proj_dirs = ProjectDirs::from("", "", "create-crx-sample").unwrap();

    let data_dir_path = proj_dirs.data_dir();

    println!("data_dir_path {:?}", &data_dir_path);

    if !data_dir_path.exists() {
        println!("create app data dir");
        create_dir(data_dir_path).unwrap();
    }
    let repo_data_dir_path = data_dir_path.join(repo_name);
    println!("repo_data_dir_path {:?}", &repo_data_dir_path);

    if !repo_data_dir_path.exists() {
        println!("Download sample");
        Command::new("git")
            .arg("clone")
            .arg(repo_link)
            .current_dir(data_dir_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    } else if update {
        println!("Update local sample");
        Command::new("git")
            .arg("pull")
            .current_dir(&repo_data_dir_path)
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
    println!("finished");

    if !repo_data_dir_path.exists() {
        panic!("Sample download fail")
    }
}
