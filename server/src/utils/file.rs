use actix_multipart::Multipart;
use actix_web::mime;
use futures_util::TryStreamExt;
use std::collections::HashMap;

pub async fn read_image_from_multipart(
    payload: &mut Multipart,
    max_file_size: usize,
) -> Result<(HashMap<String, String>, Vec<u8>), String> {
    let mut file_bytes = Vec::with_capacity(max_file_size);
    let mut metadata = HashMap::new();

    while let Some(mut field) = payload.try_next().await.map_err(|e| e.to_string())? {
        let content_disposition = field
            .content_disposition()
            .ok_or("unknow error".to_string())?;
        let field_name = content_disposition.get_name().unwrap_or("");

        match field_name {
            "file" => {
                let field_type = field.content_type().ok_or("unknow error".to_string())?;
                let is_image = field_type.type_() == mime::IMAGE;

                if !is_image {
                    return Err("the payload is not an image.".to_string());
                }

                while let Some(chunk) = field.try_next().await.map_err(|e| e.to_string())? {
                    if file_bytes.len() + chunk.len() > max_file_size {
                        return Err("the file exceeded the maximum size.".to_string());
                    }

                    file_bytes.extend_from_slice(&chunk);
                }
            }

            "metadata" => {
                let mut json_bytes = Vec::new();
                while let Some(chunk) = field.try_next().await.map_err(|e| e.to_string())? {
                    json_bytes.extend_from_slice(&chunk);
                }

                metadata = serde_json::from_slice(&json_bytes).unwrap();
            }

            _ => continue,
        }
    }

    Ok((metadata, file_bytes))
}
