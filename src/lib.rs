use sailfish::TemplateOnce;
use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct IndexTemplate {
    name: String
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_wasm_within_wasm(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let template = IndexTemplate { name: String::from("WASM!") };

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(template.render_once().unwrap())
        .build())
}
