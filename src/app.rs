use gloo::file::{callbacks::FileReader, Blob, File};
use std::collections::HashMap;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{DataTransfer, DragEvent, Event, HtmlInputElement};
use yew::prelude::*;

pub struct QrDecoder {
    readers: HashMap<String, FileReader>,
    result: Option<Result<String, String>>,
    is_over: bool,
    _paste_listener: Closure<dyn Fn(Event)>,
}

pub enum Msg {
    Files(Vec<File>),
    Blobs(Vec<Blob>),
    Loaded(String, Vec<u8>),
    DragOver(bool),
}

fn decode_qr(bytes: &[u8]) -> Result<String, String> {
    let img = image::load_from_memory(bytes)
        .map_err(|e| format!("Failed to load image: {}", e))?
        .to_luma8();

    let mut prepared = rqrr::PreparedImage::prepare(img);
    let grids = prepared.detect_grids();

    if grids.is_empty() {
        return Err("No QR code found in image".to_string());
    }

    let (_meta, content) = grids[0]
        .decode()
        .map_err(|e| format!("Failed to decode QR: {:?}", e))?;

    Ok(content)
}

fn get_clipboard_data(event: &Event) -> Option<DataTransfer> {
    // Access clipboardData via JS reflection since ClipboardEvent is unstable
    let event_obj: &js_sys::Object = event.unchecked_ref();
    js_sys::Reflect::get(event_obj, &"clipboardData".into())
        .ok()
        .and_then(|val: wasm_bindgen::JsValue| val.dyn_into::<DataTransfer>().ok())
}

impl Component for QrDecoder {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let paste_listener = Closure::new(move |event: Event| {
            event.prevent_default();
            let mut blobs = Vec::new();
            if let Some(data_transfer) = get_clipboard_data(&event) {
                // Try items for clipboard images (works for screenshots)
                let items = data_transfer.items();
                for i in 0..items.length() {
                    if let Some(item) = items.get(i) {
                        let kind = item.kind();
                        let item_type = item.type_();
                        if kind == "file" && item_type.starts_with("image/") {
                            if let Ok(Some(file)) = item.get_as_file() {
                                blobs.push(Blob::from(file));
                            }
                        }
                    }
                }
            }
            if !blobs.is_empty() {
                link.send_message(Msg::Blobs(blobs));
            }
        });

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        document
            .add_event_listener_with_callback("paste", paste_listener.as_ref().unchecked_ref())
            .unwrap();

        Self {
            readers: HashMap::new(),
            result: None,
            is_over: false,
            _paste_listener: paste_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                for file in files {
                    let name = file.name();
                    let link = ctx.link().clone();
                    let reader = gloo::file::callbacks::read_as_bytes(&file, move |res| {
                        if let Ok(bytes) = res {
                            link.send_message(Msg::Loaded(name.clone(), bytes));
                        }
                    });
                    self.readers.insert(file.name(), reader);
                }
                false
            }
            Msg::Blobs(blobs) => {
                for (i, blob) in blobs.into_iter().enumerate() {
                    let name = format!("clipboard_{}", i);
                    let link = ctx.link().clone();
                    let name_clone = name.clone();
                    let reader = gloo::file::callbacks::read_as_bytes(&blob, move |res| {
                        if let Ok(bytes) = res {
                            link.send_message(Msg::Loaded(name_clone.clone(), bytes));
                        }
                    });
                    self.readers.insert(name, reader);
                }
                false
            }
            Msg::Loaded(name, bytes) => {
                self.readers.remove(&name);
                self.result = Some(decode_qr(&bytes));
                true
            }
            Msg::DragOver(over) => {
                self.is_over = over;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_drag_over = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            Msg::DragOver(true)
        });

        let on_drag_leave = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            Msg::DragOver(false)
        });

        let on_drop = ctx.link().callback(|event: DragEvent| {
            event.prevent_default();
            let mut files = Vec::new();
            if let Some(data_transfer) = event.data_transfer() {
                if let Some(file_list) = data_transfer.files() {
                    for i in 0..file_list.length() {
                        if let Some(file) = file_list.get(i) {
                            files.push(File::from(file));
                        }
                    }
                }
            }
            Msg::Files(files)
        });

        let on_change = ctx.link().callback(|event: Event| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let mut files = Vec::new();
            if let Some(file_list) = input.files() {
                for i in 0..file_list.length() {
                    if let Some(file) = file_list.get(i) {
                        files.push(File::from(file));
                    }
                }
            }
            Msg::Files(files)
        });

        let drop_style = if self.is_over {
            "border: 3px dashed #bd93f9; background-color: #44475a; padding: 60px 40px; text-align: center; cursor: pointer; transition: all 0.2s ease;"
        } else {
            "border: 3px dashed #6272a4; background-color: #282a36; padding: 60px 40px; text-align: center; cursor: pointer; transition: all 0.2s ease;"
        };

        html! {
            <div class="drac-box" style="max-width: 600px; margin: 40px auto; padding: 20px;">
                <h1 class="drac-heading drac-heading-xl drac-text-white" style="text-align: center; margin-bottom: 30px;">
                    {"QR Code Decoder"}
                </h1>

                <div
                    style={drop_style}
                    ondragover={on_drag_over}
                    ondragleave={on_drag_leave}
                    ondrop={on_drop}
                >
                    <p class="drac-text drac-text-white" style="font-size: 1.2em; margin-bottom: 20px;">
                        {"Drop or paste QR image here"}
                    </p>
                    <p class="drac-text drac-text-grey" style="margin-bottom: 20px;">
                        {"or"}
                    </p>
                    <label class="drac-btn drac-bg-purple" style="cursor: pointer;">
                        {"Choose File"}
                        <input
                            type="file"
                            accept="image/*"
                            onchange={on_change}
                            style="display: none;"
                        />
                    </label>
                    <p class="drac-text drac-text-grey" style="margin-top: 20px; font-size: 0.9em;">
                        {"Tip: Press Ctrl+V to paste from clipboard"}
                    </p>
                </div>

                {self.view_result()}
            </div>
        }
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let _ = document.remove_event_listener_with_callback(
            "paste",
            self._paste_listener.as_ref().unchecked_ref(),
        );
    }
}

impl QrDecoder {
    fn view_result(&self) -> Html {
        match &self.result {
            Some(Ok(content)) => html! {
                <div class="drac-box drac-bg-green-transparent" style="margin-top: 20px; padding: 20px; border-radius: 8px;">
                    <p class="drac-text drac-text-green" style="font-weight: bold; margin-bottom: 10px;">
                        {"Decoded Content:"}
                    </p>
                    <p class="drac-text drac-text-white" style="word-break: break-all; font-family: monospace;">
                        {content}
                    </p>
                </div>
            },
            Some(Err(error)) => html! {
                <div class="drac-box drac-bg-red-transparent" style="margin-top: 20px; padding: 20px; border-radius: 8px;">
                    <p class="drac-text drac-text-red" style="font-weight: bold; margin-bottom: 10px;">
                        {"Error:"}
                    </p>
                    <p class="drac-text drac-text-white">
                        {error}
                    </p>
                </div>
            },
            None => html! {},
        }
    }
}
