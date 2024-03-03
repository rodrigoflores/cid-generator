use log::{info};

use proxy_wasm::traits::Context;
use proxy_wasm::traits::HttpContext;
use proxy_wasm::types::Action;
use proxy_wasm::types::LogLevel;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct RandomStringGenerator {
    context_id: u32,
    seed: u32,
}

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|context_id, _| -> Box<dyn HttpContext> {
        Box::new(RandomStringGenerator { context_id: context_id, seed: 3231 })
    });
}

impl Context for RandomStringGenerator {}

impl HttpContext for RandomStringGenerator {
    fn on_http_request_headers(&mut self, _:usize, _: bool) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            info!("In WASM : #{} -> {}: {}", self.context_id, name, value);
            info!("Seed: {}", self.seed);
        }
        Action::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
