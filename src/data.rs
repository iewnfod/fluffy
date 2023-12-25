use std::process::{Command, ExitStatus};

const DOWNLOAD_URL: &str = "https://github.com/krille-chan/fluffychat/releases/latest/download/fluffychat-web.tar.gz";

pub fn download(save_dir: &String) -> ExitStatus {
    let mut download = Command::new("wget");
    download.arg("-P").arg(save_dir);
    download.arg(DOWNLOAD_URL);
    println!("{:?}", download);
    let mut child = download.spawn().unwrap();
    let result = child.wait().unwrap();
    return result;
}

pub fn extract(target_path: &String, save_dir: &String) -> ExitStatus {
    let mut extract = Command::new("tar");
    extract.arg("-xvf").arg(target_path);
    extract.arg("-C").arg(save_dir);
    println!("{:?}", extract);
    let mut child = extract.spawn().unwrap();
    let result = child.wait().unwrap();
    return result;
}
