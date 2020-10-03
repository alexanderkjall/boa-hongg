#[macro_use]
extern crate honggfuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(s) = std::str::from_utf8(data) {
                let mut context = boa::Context::new();

                let _ = context.eval(s);
             }
        });
    }
}
