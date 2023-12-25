use std::fs::{create_dir_all, remove_file};
use std::path::PathBuf;
use std::process::Command;
use std::string::ToString;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpServer, web};
use clap::{Parser, Subcommand};
use lazy_mut::lazy_mut;
use users::{get_current_uid, get_user_by_uid};
use users::os::unix::UserExt;

mod data;

const APP_ID: &str = "com.iewnfod.fluffy";
lazy_mut! {
    static mut WEB_DIR: PathBuf = PathBuf::new();
}

async fn index(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let mut filename: PathBuf = req.match_info().query("filename").parse().unwrap();
    if filename.to_str().unwrap() == "" {
        filename = PathBuf::from("index.html");
    }
    let web_dir = unsafe { WEB_DIR.clone().unwrap() };
    let path = web_dir.join(filename);
    println!("Get File: {:?}", &path);
    Ok(NamedFile::open(path)?)
}

#[derive(Parser, Debug)]
#[command(
    author = "iewnfod",
    version = "0.1.0",
    about = "A script to start up FluffyChat web server on your own computer",
    long_about = None
)]
struct Args {
    #[arg(short, long, default_value_t = 63381)]
    port: u16,

    #[arg(short, long, default_value_t = false)]
    silent: bool,

    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Update {}
}

fn update(current_dir: &PathBuf, current_dir_str: &String) -> Result<(), String> {
    let target_path = current_dir.join("fluffychat-web.tar.gz");
    let target_path_str = target_path.as_os_str().to_str().unwrap().to_string();

    if target_path.exists() {
        remove_file(&target_path).unwrap();
    }

    println!("Downloading Resources...");
    if !data::download(current_dir_str).success() {
        return Err("Download failed!".to_string());
    }

    println!("Extracting Resources...");
    if !data::extract(&target_path_str, current_dir_str).success() {
        return Err("Extract failed!".to_string());
    }

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let current_user = get_user_by_uid(get_current_uid()).unwrap();
    let home_dir = current_user.home_dir();
    let current_dir = home_dir.join("Library").join("Application Support").join(APP_ID);
    if !current_dir.exists() { create_dir_all(&current_dir).unwrap(); }
    let current_dir_str = current_dir.as_os_str().to_str().unwrap().to_string();
    let web_dir = current_dir.join("build").join("web");

    let args = Args::parse();
    if let Some(command) = args.command {
        match command {
            Commands::Update {} => {
                update(&current_dir, &current_dir_str).unwrap();
            }
        }
    }
    
    unsafe { WEB_DIR = lazy_mut::LazyMut::Value(web_dir.clone()); }
    // 如果不存在，那就下载
    if !web_dir.exists() {
        update(&current_dir, &current_dir_str).unwrap();
    }

    let host = args.host;
    let port = args.port;

    println!("Starting FluffyChat Server...");
    println!("Listening on {}:{}", host, port);

    match HttpServer::new(|| {
        App::new().route("/{filename:.*}", web::get().to(index))
    }).bind((host.clone(), port)) {
        Ok(server) => {
            if !args.silent {
                let mut command = Command::new("open");
                command.arg(format!("http://{}:{}/", &host, &port));
                command.spawn().unwrap();
            }
            server.run().await
        },
        Err(e) => {
            return Err(e);
        }
    }
}
