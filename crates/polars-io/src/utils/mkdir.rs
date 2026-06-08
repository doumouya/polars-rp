use std::io;

use polars_utils::pl_path::PlRefPath;

pub fn mkdir_recursive(path: &PlRefPath) -> io::Result<()> {
    if !path.has_scheme() {
        std::fs::DirBuilder::new().recursive(true).create(
            path.parent()
                .ok_or(io::Error::other("path is not a file"))?,
        )?;
    }

    Ok(())
}

// RedPash wasm patch: tokio::fs has no wasm32 backend; the async mkdir is only
// used by the (wasm-gated) async-IO writers.
#[cfg(not(target_family = "wasm"))]
pub async fn tokio_mkdir_recursive(path: &PlRefPath) -> io::Result<()> {
    if !path.has_scheme() {
        tokio::fs::DirBuilder::new()
            .recursive(true)
            .create(
                path.parent()
                    .ok_or(io::Error::other("path is not a file"))?,
            )
            .await?;
    }

    Ok(())
}
