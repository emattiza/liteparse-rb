use std::collections::HashMap;
use std::sync::Arc;

use magnus::typed_data::Obj;
use magnus::{function, method, Error, RArray, RHash, RString, Ruby, TryConvert};
use magnus::prelude::*;
use tokio::sync::Mutex;

use liteparse::config::{ImageMode, LiteParseConfig, OutputFormat};
use liteparse::types::PdfInput;

fn kwarg<T: magnus::TryConvert>(kwargs: &RHash, key: &str) -> Option<T> {
    let sym = Ruby::get().unwrap().to_symbol(key);
    kwargs.get::<magnus::Symbol>(sym).and_then(|v| <T as magnus::TryConvert>::try_convert(v).ok())
}

fn kwarg_bool(kwargs: &RHash, key: &str) -> Option<bool> {
    let sym = Ruby::get().unwrap().to_symbol(key);
    kwargs.get::<magnus::Symbol>(sym).and_then(|v| {
        use magnus::value::ReprValue;
        if v.is_nil() { None } else { Some(v.to_bool()) }
    })
}

// ---------------------------------------------------------------------------
// Conversion helpers
// ---------------------------------------------------------------------------

fn screenshots_to_ruby(results: Vec<liteparse::parser::ScreenshotResult>) -> RArray {
    let ruby = Ruby::get().unwrap();
    let ary = ruby.ary_new();
    for r in results {
        let sr = ScreenshotResult {
            page_num: r.page_num,
            width: r.width,
            height: r.height,
            image_bytes: r.image_bytes,
        };
        let _ = ary.push(sr);
    }
    ary
}

// ---------------------------------------------------------------------------
// Ruby type wrappers
// ---------------------------------------------------------------------------

#[magnus::wrap(class = "LiteParse::TextItem")]
#[derive(Clone)]
struct TextItem {
    text: String,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    font_name: Option<String>,
    font_size: Option<f64>,
    confidence: Option<f64>,
}

impl TextItem {
    fn from_rust(item: liteparse::types::TextItem) -> Self {
        Self {
            text: item.text,
            x: item.x as f64,
            y: item.y as f64,
            width: item.width as f64,
            height: item.height as f64,
            font_name: item.font_name,
            font_size: item.font_size.map(|v| v as f64),
            confidence: item.confidence.map(|v| v as f64).or(Some(1.0)),
        }
    }

    fn to_rust(&self) -> liteparse::types::TextItem {
        liteparse::types::TextItem {
            text: self.text.clone(),
            x: self.x as f32,
            y: self.y as f32,
            width: self.width as f32,
            height: self.height as f32,
            font_name: self.font_name.clone(),
            font_size: self.font_size.map(|v| v as f32),
            confidence: self.confidence.map(|v| v as f32),
            ..Default::default()
        }
    }

    fn text(&self) -> &str { &self.text }
    fn x(&self) -> f64 { self.x }
    fn y(&self) -> f64 { self.y }
    fn width(&self) -> f64 { self.width }
    fn height(&self) -> f64 { self.height }
    fn font_name(&self) -> Option<&str> { self.font_name.as_deref() }
    fn font_size(&self) -> Option<f64> { self.font_size }
    fn confidence(&self) -> Option<f64> { self.confidence }
    fn inspect(&self) -> String {
        format!("#<LiteParse::TextItem text={:?} x={} y={} width={} height={}>",
            self.text, self.x, self.y, self.width, self.height)
    }
}

#[magnus::wrap(class = "LiteParse::ParsedPage")]
#[derive(Clone)]
struct ParsedPage {
    page_num: u32,
    width: f64,
    height: f64,
    text: String,
    text_items: Vec<TextItem>,
}

impl ParsedPage {
    fn from_rust(page: liteparse::types::ParsedPage) -> Self {
        Self {
            page_num: page.page_number as u32,
            width: page.page_width as f64,
            height: page.page_height as f64,
            text: page.text,
            text_items: page.text_items.into_iter().map(TextItem::from_rust).collect(),
        }
    }

    fn page_num(&self) -> u32 { self.page_num }
    fn width(&self) -> f64 { self.width }
    fn height(&self) -> f64 { self.height }
    fn text(&self) -> &str { &self.text }
    fn text_items(&self) -> RArray {
        let ruby = Ruby::get().unwrap();
        let ary = ruby.ary_new();
        for item in &self.text_items {
            let _ = ary.push(item.clone());
        }
        ary
    }
    fn inspect(&self) -> String {
        format!("#<LiteParse::ParsedPage page_num={} width={} height={} text_items_len={}>",
            self.page_num, self.width, self.height, self.text_items.len())
    }
}

#[magnus::wrap(class = "LiteParse::ParseResult")]
#[derive(Clone)]
struct ParseResult {
    pages: Vec<ParsedPage>,
    text: String,
    images: Vec<ExtractedImage>,
}

impl ParseResult {
    fn from_rust(result: liteparse::parser::ParseResult) -> Self {
        Self {
            pages: result.pages.into_iter().map(ParsedPage::from_rust).collect(),
            text: result.text,
            images: result.images.into_iter().map(ExtractedImage::from_rust).collect(),
        }
    }

    fn pages(&self) -> RArray {
        let ruby = Ruby::get().unwrap();
        let ary = ruby.ary_new();
        for page in &self.pages {
            let _ = ary.push(page.clone());
        }
        ary
    }
    fn text(&self) -> &str { &self.text }
    fn images(&self) -> RArray {
        let ruby = Ruby::get().unwrap();
        let ary = ruby.ary_new();
        for img in &self.images {
            let _ = ary.push(img.clone());
        }
        ary
    }
    fn num_pages(&self) -> usize { self.pages.len() }
    fn get_page(&self, page_num: u32) -> Option<ParsedPage> {
        self.pages.iter().find(|p| p.page_num == page_num).cloned()
    }
    fn inspect(&self) -> String {
        format!("#<LiteParse::ParseResult pages={} text_len={} images={}>",
            self.pages.len(), self.text.len(), self.images.len())
    }
}

#[magnus::wrap(class = "LiteParse::ExtractedImage")]
#[derive(Clone)]
struct ExtractedImage {
    id: String,
    page: u32,
    format: String,
    bytes: Vec<u8>,
}

impl ExtractedImage {
    fn from_rust(img: liteparse::types::ExtractedImage) -> Self {
        Self { id: img.id, page: img.page, format: img.format, bytes: img.bytes }
    }

    fn id(&self) -> &str { &self.id }
    fn page(&self) -> u32 { self.page }
    fn format(&self) -> &str { &self.format }
    fn inspect(&self) -> String {
        format!("#<LiteParse::ExtractedImage id={:?} page={} format={:?} bytes_len={}>",
            self.id, self.page, self.format, self.bytes.len())
    }
}

fn extracted_image_bytes(rb_self: &ExtractedImage) -> RString {
    Ruby::get().expect("Ruby not available").str_from_slice(&rb_self.bytes)
}

#[magnus::wrap(class = "LiteParse::ScreenshotResult")]
#[derive(Clone)]
struct ScreenshotResult {
    page_num: u32,
    width: u32,
    height: u32,
    image_bytes: Vec<u8>,
}

impl ScreenshotResult {
    fn page_num(&self) -> u32 { self.page_num }
    fn width(&self) -> u32 { self.width }
    fn height(&self) -> u32 { self.height }
    fn inspect(&self) -> String {
        format!("#<LiteParse::ScreenshotResult page_num={} width={} height={}>",
            self.page_num, self.width, self.height)
    }
}

fn screenshot_result_image_bytes(rb_self: &ScreenshotResult) -> RString {
    Ruby::get().expect("Ruby not available").str_from_slice(&rb_self.image_bytes)
}

#[magnus::wrap(class = "LiteParse::Config")]
#[derive(Clone)]
struct Config {
    ocr_language: String,
    ocr_enabled: bool,
    ocr_server_url: Option<String>,
    ocr_server_headers: Option<HashMap<String, String>>,
    tessdata_path: Option<String>,
    max_pages: usize,
    target_pages: Option<String>,
    dpi: f64,
    output_format: String,
    preserve_very_small_text: bool,
    password: Option<String>,
    quiet: bool,
    num_workers: usize,
}

impl Config {
    fn from_rust(cfg: &LiteParseConfig) -> Self {
        Self {
            ocr_language: cfg.ocr_language.clone(),
            ocr_enabled: cfg.ocr_enabled,
            ocr_server_url: cfg.ocr_server_url.clone(),
            ocr_server_headers: if cfg.ocr_server_headers.is_empty() { None } else {
                Some(cfg.ocr_server_headers.iter().cloned().collect()) },
            tessdata_path: cfg.tessdata_path.clone(),
            max_pages: cfg.max_pages,
            target_pages: cfg.target_pages.clone(),
            dpi: cfg.dpi as f64,
            output_format: match cfg.output_format {
                OutputFormat::Json => "json".to_string(),
                OutputFormat::Text => "text".to_string(),
                OutputFormat::Markdown => "markdown".to_string(),
            },
            preserve_very_small_text: cfg.preserve_very_small_text,
            password: cfg.password.clone(),
            quiet: cfg.quiet,
            num_workers: cfg.num_workers,
        }
    }

    fn ocr_language(&self) -> &str { &self.ocr_language }
    fn ocr_enabled(&self) -> bool { self.ocr_enabled }
    fn ocr_server_url(&self) -> Option<&str> { self.ocr_server_url.as_deref() }
    fn ocr_server_headers(&self) -> Option<RHash> {
        self.ocr_server_headers.clone().map(|h| {
            let hash = Ruby::get().unwrap().hash_new();
            for (k, v) in &h {
                let _ = hash.aset(k.as_str(), v.as_str());
            }
            hash
        })
    }
    fn tessdata_path(&self) -> Option<&str> { self.tessdata_path.as_deref() }
    fn max_pages(&self) -> usize { self.max_pages }
    fn target_pages(&self) -> Option<&str> { self.target_pages.as_deref() }
    fn dpi(&self) -> f64 { self.dpi }
    fn output_format(&self) -> &str { &self.output_format }
    fn preserve_very_small_text(&self) -> bool { self.preserve_very_small_text }
    fn password(&self) -> Option<&str> { self.password.as_deref() }
    fn quiet(&self) -> bool { self.quiet }
    fn num_workers(&self) -> usize { self.num_workers }
    fn inspect(&self) -> String {
        format!("#<LiteParse::Config ocr_enabled={} dpi={} max_pages={}>",
            self.ocr_enabled, self.dpi, self.max_pages)
    }
}

// ---------------------------------------------------------------------------
// Main LiteParse class
// ---------------------------------------------------------------------------

struct Inner {
    parser: liteparse::parser::LiteParse,
    config: LiteParseConfig,
    runtime: tokio::runtime::Runtime,
}

#[magnus::wrap(class = "LiteParse::LiteParse")]
struct LiteParse {
    inner: Arc<Mutex<Inner>>,
}

impl LiteParse {
    fn new(kwargs: Option<RHash>) -> Result<Self, Error> {
        // Start with upstream defaults then apply kwargs
        let mut cfg = LiteParseConfig {
            ocr_enabled: true,
            dpi: 150.0,
            max_pages: 1000,
            ..Default::default()
        };

        if let Some(ref kwargs) = kwargs {
            if let Some(v) = kwarg::<String>(kwargs, "ocr_language") { cfg.ocr_language = v; }
            if let Some(v) = kwarg_bool(kwargs, "ocr_enabled") { cfg.ocr_enabled = v; }
            if let Some(v) = kwarg::<String>(kwargs, "ocr_server_url") { cfg.ocr_server_url = Some(v); }
            if let Some(v) = kwarg::<HashMap<String, String>>(kwargs, "ocr_server_headers") {
                cfg.ocr_server_headers = v.into_iter().collect();
            }
            if let Some(v) = kwarg::<String>(kwargs, "tessdata_path") { cfg.tessdata_path = Some(v); }
            if let Some(v) = kwarg::<usize>(kwargs, "max_pages") { cfg.max_pages = v; }
            if let Some(v) = kwarg::<String>(kwargs, "target_pages") { cfg.target_pages = Some(v); }
            if let Some(v) = kwarg::<f64>(kwargs, "dpi") { cfg.dpi = v as f32; }
            if let Some(v) = kwarg::<String>(kwargs, "output_format") {
                cfg.output_format = match v.as_str() {
                    "text" => OutputFormat::Text,
                    "markdown" | "md" => OutputFormat::Markdown,
                    _ => OutputFormat::Json,
                };
            }
            if let Some(v) = kwarg_bool(kwargs, "preserve_very_small_text") { cfg.preserve_very_small_text = v; }
            if let Some(v) = kwarg::<String>(kwargs, "password") { cfg.password = Some(v); }
            if let Some(v) = kwarg_bool(kwargs, "quiet") { cfg.quiet = v; }
            if let Some(v) = kwarg::<usize>(kwargs, "num_workers") { cfg.num_workers = v; }
            if let Some(v) = kwarg::<String>(kwargs, "image_mode") {
                cfg.image_mode = match v.as_str() {
                    "off" | "none" => ImageMode::Off,
                    "embed" => ImageMode::Embed,
                    _ => ImageMode::Placeholder,
                };
            }
            if let Some(v) = kwarg_bool(kwargs, "extract_links") { cfg.extract_links = v; }
        }

        let parser = liteparse::parser::LiteParse::new(cfg.clone());
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| Error::new(Ruby::get().unwrap().exception_runtime_error(), e.to_string()))?;

        Ok(Self { inner: Arc::new(Mutex::new(Inner { parser, config: cfg, runtime })) })
    }

    fn parse(&self, input: String) -> Result<ParseResult, Error> {
        let pdf_input = PdfInput::Path(input);
        let locked = self.inner.blocking_lock();
        let result = locked.runtime.block_on(locked.parser.parse_input(pdf_input))
            .map_err(|e| runtime_err(e.to_string()))?;
        Ok(ParseResult::from_rust(result))
    }

    fn parse_bytes(&self, data: Vec<u8>) -> Result<ParseResult, Error> {
        let pdf_input = PdfInput::Bytes(data);
        let locked = self.inner.blocking_lock();
        let result = locked.runtime.block_on(locked.parser.parse_input(pdf_input))
            .map_err(|e| runtime_err(e.to_string()))?;
        Ok(ParseResult::from_rust(result))
    }

    fn screenshot(&self, input: String, page_numbers: Option<Vec<u32>>) -> Result<RArray, Error> {
        let locked = self.inner.blocking_lock();
        let results = locked.runtime.block_on(locked.parser.screenshot(&input, page_numbers))
            .map_err(|e| runtime_err(e.to_string()))?;
        Ok(screenshots_to_ruby(results))
    }

    fn config(&self) -> Config {
        Config::from_rust(&self.inner.blocking_lock().config)
    }

    fn inspect(&self) -> String {
        let inner = self.inner.blocking_lock();
        format!("#<LiteParse::LiteParse ocr_enabled={} dpi={} max_pages={}>",
            inner.config.ocr_enabled, inner.config.dpi, inner.config.max_pages)
    }
}

fn search_items(items: RArray, phrase: String, case_sensitive: Option<bool>) -> RArray {
    let case_sensitive = case_sensitive.unwrap_or(false);
    let ruby = Ruby::get().unwrap();
    let ary = ruby.ary_new();

    let mut rust_items: Vec<liteparse::types::TextItem> = Vec::new();
    for item_value in items.into_iter() {
        let obj: Obj<TextItem> = match Obj::<TextItem>::try_convert(item_value) {
            Ok(v) => v,
            _ => continue,
        };
        rust_items.push(obj.to_rust());
    }

    let options = liteparse::search::SearchOptions { phrase, case_sensitive };
    let matches = liteparse::search::search_items(&rust_items, &options);
    for m in matches {
        let _ = ary.push(TextItem::from_rust(m));
    }
    ary
}

fn run_cli(_args: Vec<String>) -> Result<(), Error> {
    Err(runtime_err("CLI not available in Ruby gem."))
}

fn runtime_err(msg: impl ToString) -> Error {
    Error::new(Ruby::get().unwrap().exception_runtime_error(), msg.to_string())
}

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

/// Exposed so the ext crate's `#[magnus::init]` can call it.
/// All classes / methods are registered inside here.
pub fn define_liteparse_module(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("LiteParse")?;

    // TextItem
    let text_item = module.define_class("TextItem", ruby.class_object())?;
    text_item.define_method("text", method!(TextItem::text, 0))?;
    text_item.define_method("x", method!(TextItem::x, 0))?;
    text_item.define_method("y", method!(TextItem::y, 0))?;
    text_item.define_method("width", method!(TextItem::width, 0))?;
    text_item.define_method("height", method!(TextItem::height, 0))?;
    text_item.define_method("font_name", method!(TextItem::font_name, 0))?;
    text_item.define_method("font_size", method!(TextItem::font_size, 0))?;
    text_item.define_method("confidence", method!(TextItem::confidence, 0))?;
    text_item.define_method("inspect", method!(TextItem::inspect, 0))?;
    text_item.define_method("to_s", method!(TextItem::inspect, 0))?;

    // ParsedPage
    let parsed_page = module.define_class("ParsedPage", ruby.class_object())?;
    parsed_page.define_method("page_num", method!(ParsedPage::page_num, 0))?;
    parsed_page.define_method("width", method!(ParsedPage::width, 0))?;
    parsed_page.define_method("height", method!(ParsedPage::height, 0))?;
    parsed_page.define_method("text", method!(ParsedPage::text, 0))?;
    parsed_page.define_method("text_items", method!(ParsedPage::text_items, 0))?;
    parsed_page.define_method("inspect", method!(ParsedPage::inspect, 0))?;
    parsed_page.define_method("to_s", method!(ParsedPage::inspect, 0))?;

    // ParseResult
    let parse_result = module.define_class("ParseResult", ruby.class_object())?;
    parse_result.define_method("pages", method!(ParseResult::pages, 0))?;
    parse_result.define_method("text", method!(ParseResult::text, 0))?;
    parse_result.define_method("images", method!(ParseResult::images, 0))?;
    parse_result.define_method("num_pages", method!(ParseResult::num_pages, 0))?;
    parse_result.define_method("get_page", method!(ParseResult::get_page, 1))?;
    parse_result.define_method("inspect", method!(ParseResult::inspect, 0))?;
    parse_result.define_method("to_s", method!(ParseResult::inspect, 0))?;

    // ExtractedImage
    let extracted_image = module.define_class("ExtractedImage", ruby.class_object())?;
    extracted_image.define_method("id", method!(ExtractedImage::id, 0))?;
    extracted_image.define_method("page", method!(ExtractedImage::page, 0))?;
    extracted_image.define_method("format", method!(ExtractedImage::format, 0))?;
    extracted_image.define_method("bytes", method!(extracted_image_bytes, 0))?;
    extracted_image.define_method("inspect", method!(ExtractedImage::inspect, 0))?;
    extracted_image.define_method("to_s", method!(ExtractedImage::inspect, 0))?;

    // ScreenshotResult
    let screenshot_result = module.define_class("ScreenshotResult", ruby.class_object())?;
    screenshot_result.define_method("page_num", method!(ScreenshotResult::page_num, 0))?;
    screenshot_result.define_method("width", method!(ScreenshotResult::width, 0))?;
    screenshot_result.define_method("height", method!(ScreenshotResult::height, 0))?;
    screenshot_result.define_method("image_bytes", method!(screenshot_result_image_bytes, 0))?;
    screenshot_result.define_method("inspect", method!(ScreenshotResult::inspect, 0))?;
    screenshot_result.define_method("to_s", method!(ScreenshotResult::inspect, 0))?;

    // Config
    let config_class = module.define_class("Config", ruby.class_object())?;
    config_class.define_method("ocr_language", method!(Config::ocr_language, 0))?;
    config_class.define_method("ocr_enabled", method!(Config::ocr_enabled, 0))?;
    config_class.define_method("ocr_server_url", method!(Config::ocr_server_url, 0))?;
    config_class.define_method("ocr_server_headers", method!(Config::ocr_server_headers, 0))?;
    config_class.define_method("tessdata_path", method!(Config::tessdata_path, 0))?;
    config_class.define_method("max_pages", method!(Config::max_pages, 0))?;
    config_class.define_method("target_pages", method!(Config::target_pages, 0))?;
    config_class.define_method("dpi", method!(Config::dpi, 0))?;
    config_class.define_method("output_format", method!(Config::output_format, 0))?;
    config_class.define_method("preserve_very_small_text", method!(Config::preserve_very_small_text, 0))?;
    config_class.define_method("password", method!(Config::password, 0))?;
    config_class.define_method("quiet", method!(Config::quiet, 0))?;
    config_class.define_method("num_workers", method!(Config::num_workers, 0))?;
    config_class.define_method("inspect", method!(Config::inspect, 0))?;
    config_class.define_method("to_s", method!(Config::inspect, 0))?;

    // Main LiteParse class
    let liteparse_class = module.define_class("LiteParse", ruby.class_object())?;
    liteparse_class.define_singleton_method("new", function!(LiteParse::new, 1))?;
    liteparse_class.define_method("parse", method!(LiteParse::parse, 1))?;
    liteparse_class.define_method("parse_bytes", method!(LiteParse::parse_bytes, 1))?;
    liteparse_class.define_method("screenshot", method!(LiteParse::screenshot, 2))?;
    liteparse_class.define_method("config", method!(LiteParse::config, 0))?;
    liteparse_class.define_method("inspect", method!(LiteParse::inspect, 0))?;
    liteparse_class.define_method("to_s", method!(LiteParse::inspect, 0))?;

    // Module functions
    module.define_module_function("search_items", function!(search_items, 3))?;
    module.define_module_function("run_cli", function!(run_cli, 1))?;

    // ParseError exception
    module.define_class("ParseError", ruby.exception_exception().as_r_class())?;

    Ok(())
}