use bytes::Bytes;
use leptess::leptonica::{pix_read_mem, PixError};
use leptess::tesseract::TessApi;

#[derive(thiserror::Error, Debug)]
pub enum ImageError {
    #[error("invalid pixel `{0}` ")]
    InvalidPix(PixError),
    #[error("invalid text")]
    InvalidText,
    #[error("invalid image")]
    InvalidImage,
}

pub async fn extract_text_from_image(image: Bytes) -> Result<String, ImageError> {
    let text = tokio::task::spawn_blocking(move || {
        let mut client = TessApi::new(None, "eng+equ").expect("Couldn't create Tess API client");
        let pix = pix_read_mem(&image).map_err(|err| ImageError::InvalidPix(err))?;
        client.set_image(&pix);
        if client.recognize() != 0 {
            return Err(ImageError::InvalidImage);
        }
        client.get_utf8_text().map_err(|_| ImageError::InvalidText)
    })
        .await
        .unwrap()?;

    Ok(text
        .replace("“", "\"")
        .replace("：", ":")
        .replace("；", ";")
        .replace("`", "'"))
}
