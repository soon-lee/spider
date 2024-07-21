#[tokio::test]
async fn test() {
    let config = crate::utils::global::Config::load().await.unwrap();
    let options = config.get_options().await.unwrap();
    println!("{:?}", options);
    let net_client = crate::utils::global::NetClient::new(options);
    let user_net_accessor = crate::accessors::net::UserNetAccessor::new(std::sync::Arc::new(net_client));
    let user = user_net_accessor.user_info(&123435656u64).await.unwrap();
    println!("{:?}", user);
}