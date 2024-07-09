use std::fs;

use rouille::router;

const INDEX: &str = include_str!("../index.html");

pub fn serve() {
    rouille::start_server("127.0.0.1:8080", move |request| {
        let content = fs::read_to_string("codebase-dump.json").expect("file to exist");
        router!(request,
            (GET) ["/"] => {
                rouille::Response::html(INDEX)
            },
            (GET) ["/codebase-dump.json"] => {
                rouille::Response::from_data("application/json", content)
            },
            _ => rouille::Response::empty_404()
        )
    })
}
