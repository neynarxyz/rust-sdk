#[cfg(test)]
mod tests {
    use api::apis::user_api::{
        FetchBulkUsersByEthOrSolAddressParams, fetch_bulk_users_by_eth_or_sol_address,
    };
    use api::apis::configuration::Configuration as ApiConfig;
    use api::apis::configuration as api_config;
    use hub_api::apis::configuration;
    use hub_api::apis::configuration::Configuration;
    use hub_api::apis::message_api::{ValidateMessageParams, validate_message};
    use reqwest;
    use reqwest::Client;

    #[tokio::test]
    async fn test_verify_message() {
        let configuration = Configuration {
            base_path: "https://hub-api.neynar.com".to_string(),
            client: Client::new(),
            user_agent: Some("rust-sdk-demo".to_string()),
            api_key: Some(configuration::ApiKey {
                prefix: None,
                key: "NEYNAR_API_DOCS".to_string(),
            }),
            basic_auth: None,
            bearer_access_token: None,
            oauth_access_token: None,
        };

        // Example message body to validate
        // Example message to validate

        // TODO generate protobuf lib code to properly encode the message
        //let msgData = models::LinkAddAllOfData {
        //    fid: 966060,
        //    timestamp: 136756108,
        //    network: models::FarcasterNetwork::FarcasterNetworkMainnet,
        //    link_body: Box::new(models::LinkBody {
        //        display_timestamp: None,
        //        target_fid: 951269,
        //        r#type: models::LinkType::Follow,
        //    }),
        //    r#type: models::MessageType::MessageTypeLinkAdd,
        //};
        //let _message = models::Message {
        //    signature: hex::decode("5d9b2bf5f1e996ae7fbfcc1e36255e69885563305fb53f7fd156194140ebfd0c").expect("Invalid hexadecimal string"),
        //    hash_scheme: models::HashScheme::HashSchemeBlake3,
        //    hash: String::from("6a85d8c25daaa33f860ebf877afcfe95ac9018e5"),
        //    signature_scheme: models::SignatureScheme::SignatureSchemeEd25519,
        //    signer: todo!(),
        //    data: todo!()
        //};

        let message_body = hex::decode("0a1b080510acfb3a18cc869a412001720c0a06666f6c6c6f7718e5873a12146a85d8c25daaa33f860ebf877afcfe95ac9018e5180122406a64aaba7f8218e99e0840502267887139cf740e96951130e47923d5b29e12821a03769cf1815c92bfd7b2a6a53ef57eb80b7dff270d9ff4a69e8c836bf63d03280132205d9b2bf5f1e996ae7fbfcc1e36255e69885563305fb53f7fd156194140ebfd0c3a1b080510acfb3a18cc869a412001720c0a06666f6c6c6f7718e5873a").expect("Invalid hexadecimal string");
        let params = ValidateMessageParams { body: message_body };

        // Call the validate_message function
        let result = validate_message(&configuration, params).await;

        // Assert the result
        match result {
            Ok(response) => {
                println!("Validation successful: {:?}", response);
                assert!(response.valid); // Assuming the response has an `is_valid` field
            }
            Err(err) => {
                eprintln!("Validation failed: {:?}", err);
                panic!("Message validation failed");
            }
        }
    }
}
