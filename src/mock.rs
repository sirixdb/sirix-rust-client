#[cfg(test)]
pub mod test_mocks {
    use super::super::info::TokenData;
    use mockito::{mock, Mock};
    use serde_json::ser::to_string;

    pub fn get_token_data() -> TokenData {
        TokenData {
            access_token: "eyJhbGciOiJSUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICJDRldPbGgyWktZd2FxLVRaaGhpcS1rQlBSSlEwSGFFWjNGcEpUWTl4Y3pVIn0.eyJqdGkiOiIwZDhlNjkzMS05YTQ3LTQzZDItYTA2MS1mN2E4NjU5Mjg4ODUiLCJleHAiOjE2MDI2ODUxNDgsIm5iZiI6MCwiaWF0IjoxNjAyNjg0ODQ4LCJpc3MiOiJodHRwOi8va2V5Y2xvYWs6ODA4MC9hdXRoL3JlYWxtcy9zaXJpeGRiIiwiYXVkIjoiYWNjb3VudCIsInN1YiI6IjA0NDRkMmY0LWE4YjItNDMyNC04ZTEzLWJhZDdiZDEwNzlmNiIsInR5cCI6IkJlYXJlciIsImF6cCI6InNpcml4IiwiYXV0aF90aW1lIjowLCJzZXNzaW9uX3N0YXRlIjoiN2VlNzcwZGQtNWRiYS00NzVkLWIyMzYtNzBlZjg4OTNmMjE1IiwiYWNyIjoiMSIsInJlYWxtX2FjY2VzcyI6eyJyb2xlcyI6WyJtb2RpZnkiLCJ2aWV3Iiwib2ZmbGluZV9hY2Nlc3MiLCJjcmVhdGUiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlbGV0ZSJdfSwicmVzb3VyY2VfYWNjZXNzIjp7ImFjY291bnQiOnsicm9sZXMiOlsibWFuYWdlLWFjY291bnQiLCJtYW5hZ2UtYWNjb3VudC1saW5rcyIsInZpZXctcHJvZmlsZSJdfX0sInNjb3BlIjoicHJvZmlsZSBlbWFpbCIsImVtYWlsX3ZlcmlmaWVkIjpmYWxzZSwicHJlZmVycmVkX3VzZXJuYW1lIjoiYWRtaW4ifQ.m5wOmb4_Mp6ZuMf1SMrbLWwolVQG_mIA5WRPGZjQtPX1fIsPACd--clmGx0e9iNPoDxgSuZ-tTwp4EZxWjylH6oYjgswmZapRwGAtg6CsFXBkoNfFop2UmMQUQbkNuV9g9Mvcu1Jz39PDw7Hxc3ge_E9e-bbo-Ec5JxboQBKbKL7hK39-IFNDFssd-BQxMfsnsXx6_Y6R1jSlHTIfra751VhqIYfFZIv2ZsFLIUsMlqTc3EkokuPXn4OVszDN96uYl4ferMKLtJEYMJubLcJx42v6sFtULXkT8Q--de-bKMfDYatBSOnrB5dXppDpD1dWooXmYdgXs89ngCPaulMVw".to_string(),
            //expires_at: 1602685148604,
            // expires_in: 300,
            expires_in: 13,
            not_before_policy: 0,
            refresh_expires_in: 1800,
            refresh_token: "eyJhbGciOiJIUzI1NiIsInR5cCIgOiAiSldUIiwia2lkIiA6ICI0ZDY5N2IyZC0yOGY4LTQ2MzktYTNhNC0zMDQ3YzE1YjdlNDcifQ.eyJqdGkiOiI0OWNkMzI4ZC0xM2E4LTRlNzMtODFkYy1hOTMzMTI4NDY0N2MiLCJleHAiOjE2MDI2ODY2NDgsIm5iZiI6MCwiaWF0IjoxNjAyNjg0ODQ4LCJpc3MiOiJodHRwOi8va2V5Y2xvYWs6ODA4MC9hdXRoL3JlYWxtcy9zaXJpeGRiIiwiYXVkIjoiaHR0cDovL2tleWNsb2FrOjgwODAvYXV0aC9yZWFsbXMvc2lyaXhkYiIsInN1YiI6IjA0NDRkMmY0LWE4YjItNDMyNC04ZTEzLWJhZDdiZDEwNzlmNiIsInR5cCI6IlJlZnJlc2giLCJhenAiOiJzaXJpeCIsImF1dGhfdGltZSI6MCwic2Vzc2lvbl9zdGF0ZSI6IjdlZTc3MGRkLTVkYmEtNDc1ZC1iMjM2LTcwZWY4ODkzZjIxNSIsInJlYWxtX2FjY2VzcyI6eyJyb2xlcyI6WyJtb2RpZnkiLCJ2aWV3Iiwib2ZmbGluZV9hY2Nlc3MiLCJjcmVhdGUiLCJ1bWFfYXV0aG9yaXphdGlvbiIsImRlbGV0ZSJdfSwicmVzb3VyY2VfYWNjZXNzIjp7ImFjY291bnQiOnsicm9sZXMiOlsibWFuYWdlLWFjY291bnQiLCJtYW5hZ2UtYWNjb3VudC1saW5rcyIsInZpZXctcHJvZmlsZSJdfX0sInNjb3BlIjoicHJvZmlsZSBlbWFpbCJ9.zRreZ1Bw-Rr9PWk6kcH1r8uioaQFoYI-CLTN2oyCYxc".to_string(),
            scope: "profile email".to_string(),
            session_state: "7ee770dd-5dba-475d-b236-70ef8893f215".to_string(),
            token_type: "bearer".to_string(),
        }
    }

    pub fn mock_auth() -> Mock {
        mock("POST", "/token")
            .match_header("content-type", "application/json")
            .match_body(r#"{"username":"admin","password":"admin","grant_type":"password"}"#)
            .with_status(200)
            .with_body(to_string(&get_token_data()).unwrap())
            .create()
    }
    pub fn mock_refresh() -> Mock {
        let mut response = get_token_data();
        response.access_token = "refreshed".to_string();
        mock("POST", "/token")
            .match_header("content-type", "application/json")
            .match_body::<&str>(
                format!(
                    r#"{{"refresh_token":"{}"}}"#,
                    get_token_data().refresh_token
                )
                .as_ref(),
            )
            .with_status(200)
            .with_body(to_string(&response).unwrap())
            .create()
    }
}
