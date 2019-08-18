use js_sys::WebAssembly::Module;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[link_section = "first"]
static A: i32 = 5;
#[link_section = "first"]
static B: i32 = 10;
#[link_section = "first"]
static C: i32 = 15;

// Instead of using reflections, let's make typed array from
// ArrayBuffer manually
#[wasm_bindgen(inline_js = "\
export function getSectionContents(sections) {
    return new Int32Array(sections[0]);
}
")]
extern "C" {
    #[wasm_bindgen(js_name = getSectionContents)]
    fn get_section_contents(sections: &JsValue) -> Vec<i32>;
}

#[wasm_bindgen(js_name = showSections)]
pub fn show_sections() -> Result<(), JsValue> {
    // it's a hidden function, and not recommended to use yet
    // https://docs.rs/wasm-bindgen/0.2.49/src/wasm_bindgen/lib.rs.html#806-808
    let module = wasm_bindgen::module()
        .dyn_into::<Module>().expect("Should be a module");
    let sections = Module::custom_sections(&module, "first");
    console_log!("sections {:?}", sections);
    let contents = get_section_contents(&sections);
    console_log!("section contents {:?}", contents);
    Ok(())
}
