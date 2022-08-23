use wasm_bindgen::prelude::*;

fn get_opts() -> usvg::Options {
  let mut fontdb = usvg::fontdb::Database::new();
  fontdb.load_font_data(include_bytes!("../fonts/Bitter-Bold.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Bitter-Regular.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Inter-Bold.ttf").to_vec());
  fontdb.load_font_data(include_bytes!("../fonts/Inter-Regular.ttf").to_vec());
  fontdb.load_font_data(
    include_bytes!("../fonts/JetBrainsMono-VariableFont_wght.ttf").to_vec(),
  );
  fontdb.set_serif_family("Bitter");
  fontdb.set_sans_serif_family("Inter");
  fontdb.set_monospace_family("JetBrains Mono");
  usvg::Options {
    fontdb,
    font_family: "Bitter".to_string(),
    ..Default::default()
  }
}

fn render_as_slice(svg: &str) -> Result<Vec<u8>, js_sys::Error> {
  let opt = get_opts();

  let data = svg.as_bytes();
  let rtree = usvg::Tree::from_data(data, &opt.to_ref())
    .map_err(|err| JsValue::from(js_sys::Error::new(&err.to_string())))?;
  let pixmap_size = rtree.svg_node().size.to_screen_size();
  let mut pixmap =
    tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
      .unwrap_throw();
  let transform = tiny_skia::Transform::default();
  resvg::render(&rtree, usvg::FitTo::Original, transform, pixmap.as_mut()).unwrap_throw();
  Ok(pixmap.encode_png().unwrap())
}

#[wasm_bindgen]
pub fn render(svg: String) -> Result<js_sys::Uint8Array, js_sys::Error> {
  render_as_slice(&svg).map(|s| js_sys::Uint8Array::from(s.as_slice()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_render() {
    let actual = render_as_slice(r##"<?xml version="1.0" encoding="UTF-8"?>
    <svg width="820px" height="312px" viewBox="0 0 820 312" version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
        <title>Testing</title>
        <g id="testing" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
            <rect fill="#FFFFFF" x="0" y="0" width="820" height="312"></rect>
            <text id="test-text" font-family="sans-serif" font-size="32" font-weight="bold" fill="#111827">
                <tspan x="51" y="90">Testing Testing Testing</tspan>
            </text>
            <text id="monospace" font-family="monospace" font-size="32" font-weight="normal" fill="#2D53A4">
                <tspan x="502" y="233">Monospace</tspan>
            </text>
        </g>
    </svg>"##);
    assert!(actual.is_ok());
    let actual = actual.unwrap();
    assert_eq!(actual.len(), 9651);
  }

  #[test]
  fn test_font_faces() {
    let opt = get_opts();
    for f in opt.fontdb.faces() {
      println!("{}", f.family);
    }
  }
}
