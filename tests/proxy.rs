extern crate reqwest;

use reqwest::header::Bearer;

#[macro_use]
mod support;

#[test]
fn test_http_proxy() {
    let server = server! {
        request: b"\
            GET http://hyper.rs/prox HTTP/1.1\r\n\
            Host: hyper.rs\r\n\
            User-Agent: $USERAGENT\r\n\
            Accept: */*\r\n\
            Accept-Encoding: gzip\r\n\
            Authorization: Bearer MY_SECRET_TOKEN\r\n\
            \r\n\
            ",
        response: b"\
            HTTP/1.1 200 OK\r\n\
            Server: proxied\r\n\
            Content-Length: 0\r\n\
            \r\n\
            ";
    };

    let proxy_uri = format!("http://{}", server.addr());
    let mut proxy = reqwest::Proxy::http(&proxy_uri).unwrap();
    proxy.set_authorization(Bearer { token: "MY_SECRET_TOKEN".to_string() });

    let url = "http://hyper.rs/prox";
    let res = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .unwrap()
        .get(url)
        .send()
        .unwrap();

    assert_eq!(res.url().as_str(), url);
    assert_eq!(res.status(), reqwest::StatusCode::Ok);
    assert_eq!(res.headers().get(),
               Some(&reqwest::header::Server::new("proxied")));
}
