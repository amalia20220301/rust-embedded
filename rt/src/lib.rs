mod math {
    mod math_js {
        #[link(wasm_import_module = "Math")]
        extern "C" {
            pub fn random() -> f64;
        }
    }

    pub fn random() -> f64 {
        unsafe { math_js::random() }
    }
}

#[export_name = "add"]
pub extern "C" fn add(left: f64, right: f64) -> f64 {
    left + right + math::random()
}