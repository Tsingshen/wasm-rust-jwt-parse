use log::info;
use proxy_wasm as wasm;

use base64_url::decode;
use serde::{Serialize, Deserialize};
use url::form_urlencoded;


#[derive(Serialize, Deserialize)]
struct  Claim {
    id: u64,
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(HelloWorld { context_id })
        },
    )
}

struct HelloWorld {
    context_id: u32,
}

impl wasm::traits::Context for HelloWorld {}


impl wasm::traits::HttpContext for HelloWorld {
    fn on_http_request_headers(&mut self, num_headers: usize, _ok: bool) -> wasm::types::Action {
        info!("Got {} HTTP headers in #{}.", num_headers, self.context_id);
        let mut user_id: String = "".to_string(); 
        let path = self.get_http_request_header(":path").unwrap();
            let query_offset = path.find("?").unwrap_or(0) +1;
            if query_offset > 1 {
                let query = &path[query_offset..path.len()];
                let encoded = form_urlencoded::parse(query.as_bytes());
                for (k, v) in encoded {
                    if k == "token" {
                        user_id = parse_jwt(v.to_string());
                    }
                }
            }
        if user_id.len() > 0 {
            info!("id = {}", user_id);
            self.set_http_request_header("x-xxx-userid",Some(&format!("{}", user_id)));
        }
        wasm::types::Action::Continue
    }
}

fn parse_jwt(token: String) -> String {
    let token_vec: Vec<&str> = token.split(".").collect();
    if token_vec.len() == 2 || token_vec.len() == 3{
        let claim = token_vec[1];
        let data = decode(&claim);
        match data {
            Ok(data_ok) => {
                match String::from_utf8(data_ok) {
                    Ok(str_ok) => {
                    // parse json
                    let json_kv: Result<Claim, serde_json::Error> = serde_json::from_str(&str_ok);
                        match json_kv {
                           Ok(json) => {
                                if json.id != 0 {
                                   return json.id.to_string()
                                }
                            }
                            Err(e) => {
                                info!("{}", e)
                            }
                        };
                    }
                    Err(e) => {
                        info!("{}", e)
                    }
                };
            }
            Err(e) => {
                info!("{}", e);
            }
        }
    }
    return "".to_string()

}