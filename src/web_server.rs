use rouille::router;

pub fn serve(payload: String, server_port: u32) {
    rouille::start_server(format!("127.0.0.1:{server_port}"), move |request| {
        router!(request,
            (GET) ["/"] => {
                rouille::Response::html(&payload)
            },
            _ => rouille::Response::empty_404()
        )
    })
}
