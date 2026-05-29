use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once;

use geozero::geojson::GeoJsonReader;
use geozero::GeozeroDatasource;
use flatgeobuf::{FgbWriter, FgbWriterOptions, GeometryType};
use serde::Deserialize;

#[wasm_bindgen(start)]
pub fn init() {
    set_once();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Deserialize, Default)]
pub struct PartialFgbWriterOptions {
    pub write_index: Option<bool>,
    pub detect_type: Option<bool>,
    pub promote_to_multi: Option<bool>,
}

#[wasm_bindgen]
pub fn encode(binary: Vec<u8>, input_options: JsValue) -> Result<Vec<u8>, JsError> {    
    log("Starting encoding...");

    let input_options: PartialFgbWriterOptions =
        serde_wasm_bindgen::from_value(input_options)
            .map_err(|e| JsError::new(&e.to_string()))?;

    let options = FgbWriterOptions {
        write_index: input_options.write_index.unwrap_or(true),
        detect_type: input_options.detect_type.unwrap_or(true),
        promote_to_multi: input_options.promote_to_multi.unwrap_or(true),
        ..Default::default()
    };
    
    let mut fgb = FgbWriter::create_with_options("countries", GeometryType::Unknown, options)?;
    let mut reader = GeoJsonReader(binary.as_slice());

    reader.process(&mut fgb)?;

    let mut out: Vec<u8> = Vec::new();
    fgb.write(&mut out)?;

    Ok(out)
}
