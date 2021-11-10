use surf;

#[tokio::main]
async fn main() {
    let url = "https://github.com";
    req(url).await;
}

async fn req(url: &str) {
    let mut res = surf::get(url).await.unwrap();
    dbg!(res.body_string().await).unwrap();
}
