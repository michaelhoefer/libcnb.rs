use reqwest::blocking::Response;
use reqwest::IntoUrl;
use sha2::Digest;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use tar::Archive;
use xz::read::XzDecoder;
use xz::write::XzEncoder;

pub fn sha256(data: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(data))
}

pub fn get(uri: impl AsRef<str>) -> anyhow::Result<String> {
    let response = reqwest::blocking::get(uri.as_ref())?;
    Ok(response.text()?)
}

pub fn get_and_extract(
    uri: impl AsRef<str>,
    dst: impl AsRef<std::path::Path>,
    prefix: Option<&str>,
) -> anyhow::Result<String> {
    let response = reqwest::blocking::get(uri.as_ref())?;
    let content = io::Cursor::new(response.bytes()?);
    let sha256 = sha256(content.get_ref());
    let decompressor = XzDecoder::new(content);
    let mut archive = Archive::new(decompressor);
    let mut target_path = PathBuf::new();
    target_path.push(&dst);
    if let Some(str) = prefix {
        for r in archive.entries()?.filter(|e| e.is_ok()) {
            let mut entry = r?;
            let relative_path = entry.path()?.strip_prefix(str)?.to_owned();
            if relative_path.as_os_str() != "" {
                entry.unpack(target_path.join(relative_path))?;
            }
        }
    } else {
        archive.unpack(dst)?;
    }

    Ok(sha256)
}

pub fn put(file_path: &PathBuf, url: impl IntoUrl) -> anyhow::Result<Response, anyhow::Error> {
    let client = reqwest::blocking::Client::new();
    let file = File::open(file_path)?;
    let response = client.put(url).body(file).send()?;
    Ok(response)
}

pub fn compress_and_put(
    source_path: &PathBuf,
    url: impl IntoUrl,
) -> anyhow::Result<(), anyhow::Error> {
    let tmpdir = tempfile::tempdir().unwrap();
    let archive_file_path = tmpdir.path().join("source.tar.xz");
    compress(source_path, &archive_file_path)?;
    put(&archive_file_path, url)?;
    Ok(())
}

fn compress(path: &PathBuf, file_path: &PathBuf) -> Result<(), anyhow::Error> {
    let file = File::create(file_path)?;
    let enc = XzEncoder::new(&file, 6);
    let mut builder = tar::Builder::new(enc);
    builder.append_dir_all(".", path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_compresses_directory() {
        let app_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/dummy_app")
            .canonicalize()
            .unwrap();
        let tmpdir = tempfile::tempdir().unwrap();
        let archive_file_path = tmpdir.path().join("source.tar.xz");
        let result = compress(&app_dir, &archive_file_path);
        match result {
            Ok(_) => {
                let tar_xz = File::open(archive_file_path).unwrap();
                let tar = XzDecoder::new(tar_xz);
                let mut archive = Archive::new(tar);
                let mut iter = archive.entries().unwrap();
                assert!(iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .path()
                    .unwrap()
                    .as_os_str()
                    .eq("./"));
                assert!(iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .path()
                    .unwrap()
                    .as_os_str()
                    .eq("src"));
                assert!(iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .path()
                    .unwrap()
                    .as_os_str()
                    .eq("src/Dummy.apex"));
                assert!(iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .path()
                    .unwrap()
                    .as_os_str()
                    .eq("src/DummyTest.cls"));
                assert!(iter
                    .next()
                    .unwrap()
                    .unwrap()
                    .path()
                    .unwrap()
                    .as_os_str()
                    .eq("src/DummyFauxTest.cls"));
            }
            Err(e) => panic!("{:?}", e),
        }
    }
}
