use crate::framework::errors::AppError;
use crate::{AppRes, CONFIG};
use axum::BoxError;
use axum::body::Bytes;
use futures::{Stream, TryStreamExt};
use sha2::{Digest, Sha256};
use std::io;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader, BufWriter};
use tokio_util::io::StreamReader;
use tracing::info;

pub async fn sha256_digest(path: &PathBuf) -> AppRes<String> {
    let input = File::open(path).await?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer).await?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(format!("{:X}", digest))
}

pub fn path_is_valid(path: &str) -> bool {
    let path = std::path::Path::new(path);
    let mut components = path.components().peekable();

    if let Some(first) = components.peek() {
        if !matches!(first, std::path::Component::Normal(_)) {
            return false;
        }
    }

    components.count() == 1
}

pub async fn stream_to_file<S, E>(filename: &str, stream: S) -> AppRes<String>
where
    S: Stream<Item = Result<Bytes, E>>,
    E: Into<BoxError>,
{
    let extension_name = filename.split('.').last().unwrap_or("");

    if !path_is_valid(filename) {
        return Err(AppError::new("Invalid path"));
    }

    let hash_file_name = async {
        // Convert the stream into an `AsyncRead`.
        let body_with_io_error = stream.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        let body_reader = StreamReader::new(body_with_io_error);
        futures::pin_mut!(body_reader);

        // Create the file. `File` implements `AsyncWrite`.
        let path = std::path::Path::new(CONFIG.file_server_directory.as_str()).join(filename);
        let mut file = BufWriter::new(File::create(path.clone()).await?);

        // Copy the body into the file.
        tokio::io::copy(&mut body_reader, &mut file).await?;
        let hash = sha256_digest(&path).await?;
        let hash_file_name = format!("{hash}.{extension_name}");
        info!("file: {} has saved", hash_file_name);
        let hash_file = std::path::Path::new(CONFIG.file_server_directory.as_str())
            .join(hash_file_name.clone());

        tokio::fs::rename(path, hash_file).await?;
        Ok::<_, AppError>(hash_file_name)
    }
    .await?;
    Ok(hash_file_name)
}
