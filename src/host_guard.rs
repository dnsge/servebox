use actix_web::guard::Guard;

pub struct HostGuard(String);

pub fn new(s: &str) -> HostGuard {
    HostGuard(s.to_string())
}

impl Guard for HostGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        if let Some(val) = ctx.head().headers.get("Host") {
            return val == &self.0;
        }

        false
    }
}
