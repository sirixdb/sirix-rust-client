use super::{client::request, error::SirixError};
use crate::info::{TokenData, TokenPostData};

use log::{error, info};
use serde_json::ser::to_string;
use std::thread::sleep;
use std::time::Duration;
use std::{sync::Arc, sync::RwLock, thread::spawn};

fn authenticate(
    agent: ureq::Agent,
    endpoint: &str,
    username: &str,
    password: &str,
) -> Result<TokenData, SirixError> {
    let req = agent.post(endpoint).set("content-type", "application/json");
    let response = request::<TokenData>(
        req,
        Some(
            &to_string(&TokenPostData {
                username: username.to_string(),
                password: password.to_string(),
                grant_type: "password".to_string(),
            })
            .unwrap(),
        ),
    );
    match response {
        Ok(response) => Ok(response.body),
        Err(err) => Err(err),
    }
}

fn refresh(
    agent: ureq::Agent,
    endpoint: &str,
    token_data: TokenData,
) -> Result<TokenData, SirixError> {
    let refresh_json = format!(r#"{{"refresh_token":"{}"}}"#, token_data.refresh_token);
    let req = agent.post(endpoint).set("content-type", "application/json");
    let response = request::<TokenData>(req, Some(&refresh_json));
    match response {
        Ok(response) => Ok(response.body),
        Err(err) => Err(err),
    }
}

fn begin_authentication_loop(
    agent: ureq::Agent,
    lock: Arc<RwLock<Option<TokenData>>>,
    base_url: &str,
    username: &str,
    password: &str,
) {
    let endpoint = &format!("{}/token", base_url);
    let token_data = authenticate(agent.clone(), endpoint, username, password);
    let token_data = match token_data {
        Ok(token_data) => {
            info!("authentication with credentials successful");
            //response_sender.send(Some(token_data.clone())).unwrap();
            Some(token_data)
        }
        Err(_) => {
            error!("authentication with credentials failed");
            None
        }
    };
    let mut token_data = token_data.clone().unwrap();
    {
        let mut l = lock.write().unwrap();
        *l = Some(token_data.clone());
    }

    loop {
        sleep(Duration::from_secs(token_data.expires_in.clone() - 10));
        let refresh_response = refresh(agent.clone(), endpoint, token_data.clone());
        match refresh_response {
            Ok(new_token_data) => {
                info!("authentication with credentials successful");
                token_data = new_token_data;
                {
                    let mut l = lock.write().unwrap();
                    *l = Some(token_data.clone());
                }
            }
            Err(_) => {
                error!("authentication with credentials failed")
            }
        };
    }
}

pub fn auth(
    agent: ureq::Agent,
    base_url: &str,
    username: &str,
    password: &str,
) -> Arc<RwLock<Option<TokenData>>> {
    let base_url = base_url.to_owned();
    let username = username.to_owned();
    let password = password.to_owned();
    let lock = Arc::new(RwLock::new(None));
    let cloned_lock = Arc::clone(&lock);
    spawn(move || {
        begin_authentication_loop(agent, cloned_lock, &base_url, &username, &password);
    });
    return lock;
}

#[cfg(test)]
mod tests {
    use super::super::super::mock::test_mocks;
    use super::*;

    #[test]
    fn test_authenticate() {
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let agent = ureq::agent();

        let response = authenticate(agent, &format!("{}/token", url), "admin", "admin");
        assert_eq!(response.unwrap(), test_mocks::get_token_data());
    }

    #[test]
    fn test_refresh() {
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let _m2 = test_mocks::mock_refresh();
        let agent = ureq::agent();
        let endpoint = &format!("{}/token", url);

        let response = authenticate(agent.clone(), endpoint, "admin", "admin");
        let token_data = response.unwrap();
        assert_eq!(token_data.clone(), test_mocks::get_token_data());

        let refresh_response = refresh(agent, endpoint, token_data);
        assert_ne!(refresh_response.unwrap(), test_mocks::get_token_data());
    }

    #[test]
    fn test_auth_func() {
        let url = &mockito::server_url();
        let _m = test_mocks::mock_auth();
        let _m2 = test_mocks::mock_refresh();
        let agent = ureq::agent();

        let lock = auth(agent, url, "admin", "admin");
        sleep(Duration::from_millis(10));
        {
            assert_eq!(
                (*(lock.clone().read().unwrap())).clone().unwrap(),
                test_mocks::get_token_data()
            );
        }
        sleep(Duration::from_secs(
            test_mocks::get_token_data().expires_in - 10,
        ));
        sleep(Duration::from_millis(10));
        assert_ne!(
            (*(lock.clone().read().unwrap())).clone().unwrap(),
            test_mocks::get_token_data()
        )
    }
}
