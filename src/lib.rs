//define add_two func
fn add_two(x: i32) -> i32 {
    x + 2
}

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)] //只要标记都默认实现
#[serde(crate = "rocket::serde")]
struct ServerConfig {
    workers: u64,
    ignore: bool,
    auth_server: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_add_two() {
        assert_eq!(add_two(2), 4);
    }
    //test give value
    #[test]
    fn test_give_value() {
        let config = ServerConfig {
            workers: 1,
            ignore: true,
            auth_server: Some("http://localhost:8080".to_string()),
        };
        assert_eq!(config.workers, 1);
        assert_eq!(config.ignore, true);
        assert_eq!(config.auth_server, Some("http://localhost:8080".to_string()));
    }


}
