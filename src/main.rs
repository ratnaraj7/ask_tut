use anyhow::{Ok, Result};
use askama_axum::Template;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

#[derive(Clone)]
struct Button<'a> {
    attrs: String,
    value: &'a str,
    tag: &'a str,
}

enum ButtonType {
    Primary,
    Secondary,
}

impl<'a> Button<'a> {
    fn new(
        b_type: ButtonType,
        is_link: bool,
        value: &'a str,
        attrs: &'a str,
        ext_class: Option<&'a str>,
    ) -> Self {
        let attrs = format!(
            "{attrs} class=\"{} {}\"",
            Self::create_class(b_type),
            ext_class.unwrap_or("")
        );
        Self {
            attrs,
            value,
            tag: if is_link { "a" } else { "button" },
        }
    }

    fn create_class(b_type: ButtonType) -> String {
        let base_class = "border-black border-2 px-4 py-2 text-black";
        match b_type {
            ButtonType::Primary => format!("{base_class} bg-red-500"),
            ButtonType::Secondary => format!("{base_class} bg-green-500"),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    description: &'a str,
    title: &'a str,
    primary_btn: Button<'a>,
    secondary_btn: Button<'a>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let router = Router::new().route("/", get(index));
    let tcp_listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(tcp_listener, router).await?;
    Ok(())
}

async fn index() -> Index<'static> {
    let id = Index {
        title: "hello world",
        description: "this is hello word, nothing else!",
        primary_btn: Button::new(
            ButtonType::Primary,
            false,
            "this is primary non link",
            "onclick=\"alert('this is primary non link')\"",
            None,
        ),
        secondary_btn: Button::new(
            ButtonType::Secondary,
            true,
            "this is secondary link",
            "onclick=\"alert('this is secondary link')\"",
            Some("block w-full"),
        ),
    };

    dbg!(id.render().unwrap());

    id
}
