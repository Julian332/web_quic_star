use std::ops::Deref;
use crate::config::FILE_SERVER_DIRECTORY;
use crate::framework::api_doc::empty_resp_docs;
use crate::framework::auth::AuthBackend;
use crate::utils::file;
use crate::AppRes;
use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
use aide::OperationIo;
use axum::extract::Multipart;
use axum::response::Html;
use axum::Json;
use axum_login::login_required;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub fn upload_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route(
            "/",
            get_with(show_form, empty_resp_docs).post_with(accept_form, empty_resp_docs),
        )
        .route_layer(login_required!(AuthBackend))
}
async fn show_form() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Upload something!</title>
            </head>
            <body>
                <form action="/upload" method="post" enctype="multipart/form-data">
                    <div>
                        <label>
                            Upload file:
                            <input type="file" name="file" multiple>
                        </label>
                    </div>

                    <div>
                        <input type="submit" value="Upload files">
                    </div>
                </form>
            </body>
        </html>
        "#,
    )
}
async fn accept_form(mut multipart: Multipart) -> AppRes<Json<Vec<FileSave>>> {
    let mut res = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = if let Some(file_name) = field.file_name() {
            file_name.to_owned()
        } else {
            continue;
        };

        let hash = file::stream_to_file(&file_name, field).await?;
        res.push(FileSave {
            filename: file_name,
            hash: format!("/{}/{hash}",FILE_SERVER_DIRECTORY.deref()),
        });
    }

    Ok(Json(res))
}

#[derive(Serialize, Deserialize, JsonSchema, OperationIo)]
struct FileSave {
    filename: String,
    hash: String,
}
