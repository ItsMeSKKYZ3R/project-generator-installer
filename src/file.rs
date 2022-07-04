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

pub async fn download() -> Result<()> {
    let tmp_dir = PathBuf::new();
    let target = "https://skkyz3r.fr/password-generator.exe";
    let response = reqwest::get(target).await?;

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
        File::create(fname)?
    };
    let content =  response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}