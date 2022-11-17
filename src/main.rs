extern crate reqwest;

#[tokio::main]
async fn main() {
    let version_url =
        "https://raw.githubusercontent.com/portalmaster137/requestingupdates/main/.version";
    check_version(version_url.to_string()).await;
}

fn check_version_is_higher(file_version: String, remote_version: String) -> bool {
    let file_version = file_version.replace(".", "");
    let remote_version = remote_version.replace(".", "");
    let file_version = file_version.parse::<i32>().unwrap();
    let remote_version = remote_version.parse::<i32>().unwrap();
    if file_version < remote_version {
        return true;
    }
    return false;
}

async fn check_version(ver_url: String) {
    if (std::path::Path::new(".version").exists()) {
        //load version
        let version = std::fs::read_to_string(".version").unwrap();
        println!("Version: {}", version);
        //get remote version
        let remote = reqwest::get(ver_url).await.unwrap().text().await.unwrap();
        println!("Remote Version: {}", remote);
        //compare versions
        if check_version_is_higher(version, remote) {
            println!("Update available!");
        } else {
            println!("No update available!");
        }
    } else {
        panic!("Version file does not exist!, Please reclone the repo");
    }
}
