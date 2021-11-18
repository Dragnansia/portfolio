use mongodb::options::ClientOptions;
use mongodb::Client;
use std::process::exit;

pub async fn new_connection(uri: &str, app_name: &str) -> Client {
    let mut options = match ClientOptions::parse(&uri).await {
        Ok(options) => options,
        Err(err) => {
            println!("{:?}", err);
            exit(1);
        }
    };
    options.app_name = Some(app_name.to_string());

    let client = match Client::with_options(options) {
        Ok(client) => client,
        Err(err) => {
            println!("{:?}", err);
            exit(2);
        }
    };

    client
}
