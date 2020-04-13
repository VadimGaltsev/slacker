#![allow(dead_code)]
use reqwest::r#async::Client;
use std::sync::Arc;
use std::time::Duration;
use reqwest::header::{HeaderMap};
use futures::Future;
use crate::methods::{SlackRequest};
use reqwest::{Url, Error};
use std::str::FromStr;
use reqwest::r#async::Request;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub(crate) const CONTENT_TYPE: &'static str = "Content-Type";
pub(crate) const APPLICATION_URLENCODED: &'static str = "application/x-www-form-urlencoded";
pub(crate) const CHARSET_UTF_8: &'static str = "charset=utf-8";
pub(crate) const END_POINT_API_SLACK: &'static str = ".slack.com/api/";

#[derive(Clone)]
pub struct Slacker {
    token: String,
    endpoint: String,
    client: Arc<Client>,
}

pub struct Builder {
    token: Option<String>,
    workspace: Option<String>,
    client: Option<Arc<Client>>,
}

impl Slacker {
    pub fn builder() -> Builder {
        Builder {
            token: None,
            workspace: None,
            client: None,
        }
    }

    pub fn new(token: &str, workspace: &str) -> Slacker {
        let endpoint = format!("https://{}{}", workspace, END_POINT_API_SLACK);
        let token = token.to_owned();
        let client = Arc::new(create_new_client(&token));
        Slacker {
            token,
            endpoint,
            client,
        }
    }

    pub fn post<Response: DeserializeOwned, T: SlackRequest<Response> + Sized>(
        &self,
        request: T
    ) -> impl Future<Item=Response, Error=Error> {
        let request = self.create_post_request(request);
        self.client.execute(request).then(|resp| {
            println!("{:?}", resp);
            let mut response = resp.unwrap();
            response.json::<Response>()
        })
    }

    pub fn get<Response: DeserializeOwned, T: SlackRequest<Response> + Sized>(
        &self,
        request: T
    ) -> impl Future<Item=Response, Error=Error> {
        let request = self.create_get_request(request);
        self.client.execute(request).then(|resp| {
            println!("{:?}", resp);
            let mut response = resp.unwrap();
            response.json::<Response>()
        })
    }

    fn create_get_request<Response, T: SlackRequest<Response>>(&self, to_json: T) -> Request {
        let url = Url::from_str(&format!("{}{}{}", self.endpoint, T::METHOD_NAME, to_json.get_params())).unwrap();
        println!("{}", url);
        self.client.get(url)
            .build()
            .unwrap()
    }

    fn create_post_request<Response, T: Serialize + SlackRequest<Response>>(&self, to_json: T) -> Request {
        let url = Url::from_str(&format!("{}{}", self.endpoint, T::METHOD_NAME)).unwrap();
        println!("{}", serde_json::to_string(&to_json).unwrap());
        self.client.post(url)
            .json(&to_json)
            .build()
            .unwrap()
    }
}

impl Builder {
    fn token(mut self, token: &str) -> Builder {
        self.token = Some(token.to_owned());
        self
    }

    fn workspace(mut self, workspace: &str) -> Builder {
        self.workspace = Some(workspace.to_owned());
        self
    }

    fn client(mut self, client: Client) -> Builder {
        self.client = Some(Arc::new(client));
        self
    }

    fn build(self) -> Slacker {
        let token = if let Some(token) = self.token {
            token
        } else {
            panic!("Token cannot be null");
        };
        let workspace = if let Some(workspace) = self.workspace {
            workspace
        } else {
            panic!("Workspace cannot be null");
        };
        let client = if let Some(client) = self.client {
            client
        } else {
            Arc::new(create_new_client(&token))
        };
        Slacker {
            token,
            endpoint: workspace,
            client,
        }
    }
}

fn create_new_client(token: &String) -> Client {
    let mut headers = HeaderMap::new();
    headers.append(
        "Authorization",
        format!("Bearer {}", token).parse().unwrap(),
    );
    Client::builder()
        .cookie_store(true)
        .timeout(Duration::new(60, 0))
        .default_headers(headers)
        .build()
        .unwrap()
}