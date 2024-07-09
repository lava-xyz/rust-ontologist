use std::fs;

use rouille::router;

use crate::output::cytoscape::Repr;

const INDEX: &str = include_str!("../index.html");

pub fn serve(repr: Repr, server_port: u32) {
    rouille::start_server(format!("127.0.0.1:{}", server_port), move |request| {
        router!(request,
            (GET) ["/"] => {
                rouille::Response::html(INDEX)
            },
            (GET) ["/codebase-dump.json"] => {
                rouille::Response::json(&repr)
            },
            _ => rouille::Response::empty_404()
        )
    })
}
