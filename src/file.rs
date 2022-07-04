use error_chain::error_chain;
use std::io::copy;
use std::fs::File;
use std::path::PathBuf;
use dirs;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[cfg(target_os = "windows")]
pub async fn download() -> Result<()> {
    let tmp_dir = PathBuf::new();
    let target = "https://skkyz3r.fr/project-generator.exe";
    let response = reqwest::get(target).await.expect("Failed to execute request.");

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.join("Program Files").join("Project Generator").join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).expect("Failed to create file")
    };
    let content =  response.text().await.expect("Failed to get response text.");
    copy(&mut content.as_bytes(), &mut dest).expect("Failed to copy file to destination.");

    Ok(())
}

#[cfg(target_os = "linux")]
pub async fn download() -> Result<()> {
    let dir = dirs::executable_dir().expect("Failed to get executable files directory");
    println!("{dir:?}");
    let target = "https://skkyz3r.fr/project-generator";
    let response = reqwest::get(target).await.expect("Failed to execute request.");

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File to download: '{fname}'");

        let fname = dir.join(fname);
        println!("Will be located under: '{fname:?}'");

        File::create(fname).expect("Failed to create file")
    };

    let content = response.text().await.expect("Failed to get response text.");
    copy(&mut content.as_bytes(), &mut dest).expect("Failed to copy file to destination.");

    Ok(())
}