mod msa_auth;
mod minecraft;

#[cfg(test)]
mod tests {
    use oauth2::StandardDeviceAuthorizationResponse;

    use reqwest::Client;
    use crate::auth::minecraft::MinecraftAuthorizationFlow;
    use crate::auth::msa_auth;


    #[test]
    fn msa_test() {
        let ms_auth_flow = msa_auth::MicrosoftAuthFlow::new().unwrap();
        let detail:StandardDeviceAuthorizationResponse = ms_auth_flow.generate_msa_device_code_auth().unwrap();

        println!(
            "Open this URL in your browser:\n{}\nand enter the code: {}",
            &detail.verification_uri().to_string(),
            &detail.user_code().secret().to_string()
        );

        open::that(detail.verification_uri().to_string()).unwrap();

        let mc_flow = MinecraftAuthorizationFlow::new(Client::new());
        let (xbox_token,user_hash) = mc_flow.xbox_token(detail.user_code().secret()).unwrap();
        let xbox_xsts_token = mc_flow.xbox_security_token(xbox_token).unwrap();
        let mc_token = mc_flow.exchange_microsoft_token(user_hash,xbox_xsts_token).unwrap();
        
        println!("{}",mc_token.access_token);
    }
}