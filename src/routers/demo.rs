use crate::app_response::AppResult;
use crate::utils::dh::generate_random_prime;
use salvo::oapi::endpoint;

#[endpoint]
pub async fn hello() -> AppResult<String> {
    Ok(format!(
        "Hello World from salvo, here's a prime: {}",
        generate_random_prime()
    ))
}

#[allow(unused_imports)]
mod tests {
    use crate::config::CFG;
    use crate::get_address;
    use salvo::test::{ResponseExt, TestClient};
    use salvo::Service;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(crate::routers::router());

        let content = TestClient::get(format!(
            "http://{}",
            &get_address().replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}
