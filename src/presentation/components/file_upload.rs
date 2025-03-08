use wasm_bindgen::JsCast;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::prelude::*;
use crate::domain::entities::PdfDocument;
use crate::presentation::context::LoadingContext;

#[derive(Properties, PartialEq)]
pub struct FileUploadProps {
    pub on_file_selected: Callback<PdfDocument>,
}

#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProps) -> Html {
    let loading_ctx = use_context::<LoadingContext>().expect("No LoadingContext found");
    let drag_active = use_state(|| false);
    let input_ref = use_node_ref();

    let ondragover = {
        let drag_active = drag_active.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_active.set(true);
        })
    };

    let ondragleave = {
        let drag_active = drag_active.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            drag_active.set(false);
        })
    };

    let ondrop = {
        let loading_ctx = loading_ctx.clone();
        let props = props.clone();
        Callback::from(move |e: DragEvent| {
            e.prevent_default();
            if let Some(files) = e.data_transfer().and_then(|dt| dt.files()) {
                handle_files(files, props.on_file_selected.clone(), loading_ctx.set_loading.clone());
            }
        })
    };

    let onchange = {
        let loading_ctx = loading_ctx.clone();
        let props = props.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_: Event| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                if let Some(files) = input.files() {
                    handle_files(files, props.on_file_selected.clone(), loading_ctx.set_loading.clone());
                }
            }
        })
    };

    let onclick_upload = {
        let input_ref = input_ref.clone();
        Callback::from(move |_| {
            if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                input.click();
            }
        })
    };

    html! {
        <div 
            class={classes!(
                "border-2 border-dashed rounded-lg p-8 text-center cursor-pointer transition-colors",
                if *drag_active { "border-primary bg-blue-50 dark:bg-blue-900/20" } 
                else { "border-gray-300 dark:border-gray-600 hover:border-primary" }
            )}
            ondragover={ondragover}
            ondragleave={ondragleave}
            ondrop={ondrop}
            onclick={onclick_upload}
        >
            <input 
                type="file"
                ref={input_ref}
                accept=".pdf"
                class="hidden"
                onchange={onchange}
            />
            <div class="flex flex-col items-center justify-center space-y-4">
                <svg class="w-12 h-12 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                        d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
                </svg>
                <div class="text-lg font-medium text-gray-900 dark:text-gray-100">
                    {"Drop your PDF file here or click to upload"}
                </div>
                <p class="text-sm text-gray-500 dark:text-gray-400">
                    {"Only PDF files are supported"}
                </p>
            </div>
        </div>
    }
}

fn handle_files(files: FileList, on_file_selected: Callback<PdfDocument>, set_loading: Callback<(bool, Option<String>)>) {
    if let Some(file) = files.get(0) {
        if !file.name().to_lowercase().ends_with(".pdf") {
            // Show error for invalid file type
            set_loading.emit((false, Some("Please select a PDF file".to_string())));
            return;
        }

        set_loading.emit((true, Some("Loading PDF file...".to_string())));

        // Create a new FileReader
        let reader = web_sys::FileReader::new().unwrap();
        let reader_clone = reader.clone();
        
        let onload = Closure::wrap(Box::new(move || {
            if let Ok(result) = reader_clone.result() {
                if let Some(content) = result.as_string() {
                    let doc = PdfDocument::new(
                        file.name(),
                        content.into_bytes(),
                        None,
                    );
                    on_file_selected.emit(doc);
                }
            }
            set_loading.emit((false, None));
        }) as Box<dyn FnMut()>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.read_as_text(&file).unwrap();
        onload.forget();
    }
}
