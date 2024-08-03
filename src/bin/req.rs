fn main() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .expect("tokio runtime");

    let default_url = String::from("https://httpbin.org/ip");
    let mut args = std::env::args();
    let _ = args.next();
    let url = if let Some(url) = args.next() {
        url
    } else {
        default_url
    };

    rt.block_on(req(url));
}


async fn req(url: String) {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await.expect("client.get");
    println!("{}", res.text().await.expect("res.text"));
}
