use std::sync::Mutex;
use lazy_static::lazy_static;

pub struct LocalStorage {
    pub token: Option<String>,
    pub base_url:Option<String>,
    pub api_url:Option<String>,
    pub account_id: Option<i32>,
    pub character_id: Option<i32>,
}

impl LocalStorage {
    pub fn new() -> Self {
        LocalStorage {
            token: None,
            base_url: None,
            api_url:None,
            account_id: None,
            character_id: None,
        }
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

    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = Some(base_url);
    }

    pub fn get_base_url(&self) -> Option<&String> {
        self.base_url.as_ref()
    }

    pub fn set_account_id(&mut self, account_id: i32) {
        self.account_id = Some(account_id);
    }

    pub fn get_account_id(&self) -> Option<&i32> {
        self.account_id.as_ref()
    }

    pub fn set_character_id(&mut self, character_id: i32) {
        self.character_id = Some(character_id);
    }

    pub fn get_character_id(&self) -> Option<&i32> {
        self.character_id.as_ref()
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

pub fn set_global_base_url(base_url: String) {
    let mut storage = LOCAL_STORAGE.lock().unwrap();
    storage.set_base_url(base_url);
}


pub fn get_global_base_url() -> String {
    let storage = LOCAL_STORAGE.lock().unwrap();
    storage.get_base_url().cloned().unwrap_or("".to_string())
}

pub fn set_global_account_id(account_id: i32) {
    let mut storage = LOCAL_STORAGE.lock().unwrap();
    storage.set_account_id(account_id);
}


pub fn get_global_account_id() -> i32 {
    let storage = LOCAL_STORAGE.lock().unwrap();
    storage.get_account_id().cloned().unwrap_or(-9999)
}


pub fn set_global_character_id(character_id: i32) {
    let mut storage = LOCAL_STORAGE.lock().unwrap();
    storage.set_character_id(character_id);
}


pub fn get_global_character_id() -> i32 {
    let storage = LOCAL_STORAGE.lock().unwrap();
    storage.get_character_id().cloned().unwrap_or(-9999)
}
