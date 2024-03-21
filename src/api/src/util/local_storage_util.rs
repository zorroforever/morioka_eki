use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::util::local_storage_util;

pub struct LocalStorage {
    pub token: Option<String>,
    pub api_url:Option<String>,
}

impl LocalStorage {
    pub fn new() -> Self {
        LocalStorage { token: None,api_url:None }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&String> {
        self.token.as_ref()
    }

    pub fn set_api_url(&mut self, api_url: String) {
        self.api_url = Some(api_url);
    }

    pub fn get_api_url(&self) -> Option<&String> {
        self.api_url.as_ref()
    }
}

lazy_static! {
    static ref LOCAL_STORAGE: Mutex<LocalStorage> = Mutex::new(LocalStorage::new());
}


pub fn set_global_token(token: String) {
    let mut storage = LOCAL_STORAGE.lock().unwrap();
    storage.set_token(token);
}


pub fn get_global_token() -> String {
    let storage = LOCAL_STORAGE.lock().unwrap();
    storage.get_token().cloned().unwrap_or("".to_string())
}

pub fn refresh_global_token(new_token: String) {
    set_global_token(new_token);
}

pub fn set_global_api_url(api_url: String) {
    let mut storage = LOCAL_STORAGE.lock().unwrap();
    storage.set_api_url(api_url);
}


pub fn get_global_api_url() -> String {
    let storage = LOCAL_STORAGE.lock().unwrap();
    storage.get_api_url().cloned().unwrap_or("".to_string())
}

pub fn get_global_union_api_url_with_token() -> String {
    let storage = LOCAL_STORAGE.lock().unwrap();
    let v1 = storage.get_api_url().cloned().unwrap_or("".to_string());
    let v2= storage.get_token().cloned().unwrap_or("".to_string());
    let url = format!("{}{}",v1,v2);
    url
}

