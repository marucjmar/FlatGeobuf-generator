use wasm_bindgen::prelude::*;
use console_error_panic_hook::set_once;

use geozero::geojson::GeoJsonReader;
use geozero::GeozeroDatasource;
use flatgeobuf::FgbWriter;
use flatgeobuf::GeometryType;

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
pub fn encode(binary: Vec<u8>) -> Result<Vec<u8>, JsError> {    
    log("Starting encoding...");

    let mut fgb = FgbWriter::create("countries",GeometryType::MultiPolygon)?;
    let mut reader = GeoJsonReader(binary.as_slice());

    reader.process(&mut fgb)?;

    let mut out: Vec<u8> = Vec::new();
    fgb.write(&mut out)?;

    Ok(out)
}
