use std::path::PathBuf;

use tangram_client as tg;
use tangram_error::{WrapErr, Result};
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    if let Err(e) = main_inner().await {
        eprintln!("an error occurred: {e}");
        std::process::exit(1);
    }
}

async fn main_inner() -> Result<()> {
    let addr = tg::Address::Unix("/Users/mikedorf/.tangram/socket".into());
	let client = tg::Builder::new(addr).build();
    client.connect().await?;
    let tg = &client;

    let count = std::env::args().nth(1).wrap_err("Expected an argument.")?.parse().wrap_err("Expected an integer.")?;

    if count == 0 {
        let path = PathBuf::from("files.json");
        let file = std::fs::File::open(&path).unwrap();
        let json = serde_json::from_reader::<_, Vec<tg::artifact::Id>>(file).unwrap();
        for id in json {
            let artifact = tg::Artifact::with_id(id);
            artifact.check_out(tg, None).await?;
        }
        return Ok(())
    }

    let result = vfs_stress_test::create_files(tg, count).await?;
    let json = serde_json::to_vec(&result).wrap_err("Failed to serialize id vector.")?;

    let path = PathBuf::from("files.json");
    if path.exists() {
        tokio::fs::remove_file(&path).await.wrap_err("Failed to remove file.")?;
    }

    let mut file = tokio::fs::File::options()
        .write(true)
        .read(true)
        .create(true)
        .open(&path)
        .await
        .wrap_err("Failed to open file.")?;

    file.write_all(&json).await.wrap_err("Failed to write file.")?;
    Ok(())
}