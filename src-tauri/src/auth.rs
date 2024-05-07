mod msa_auth;
mod minecraft;

#[cfg(test)]
mod tests {
    use oauth2::StandardDeviceAuthorizationResponse;
    use oauth2::TokenResponse;
    use reqwest::Client;
    use crate::auth::minecraft::MinecraftAuthorizationFlow;
    use crate::auth::msa_auth::{create_client, generate_msa_device_code_auth, get_msa_token};


    #[test]
    fn msa_test() {
        let client = create_client().unwrap();
        let detail:StandardDeviceAuthorizationResponse = generate_msa_device_code_auth(&client).unwrap();

        println!(
            "Open this URL in your browser:\n{}\nand enter the code: {}",
            &detail.verification_uri().to_string(),
            &detail.user_code().secret().to_string()
        );

        open::that(detail.verification_uri().to_string()).unwrap();
        let token = get_msa_token(&client, &detail).unwrap().access_token().secret().to_string();

        let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
        let (xbox_token,user_hash) = mc_flow.xbox_token(token).unwrap();
        let xbox_xsts_token = mc_flow.xbox_security_token(xbox_token).unwrap();
        let mc_token = mc_flow.exchange_microsoft_token(user_hash,xbox_xsts_token).unwrap();
        
        println!("{}",mc_token.access_token);
    }
}