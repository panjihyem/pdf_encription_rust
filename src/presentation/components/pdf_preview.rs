use wasm_bindgen::JsCast;
use web_sys::{HtmlIframeElement, window};
use yew::prelude::*;
use crate::domain::entities::PdfDocument;
use crate::infrastructure::I18nService;
use crate::presentation::context::LoadingContext;

#[derive(Properties, PartialEq)]
pub struct PdfPreviewProps {
    pub document: PdfDocument,
    pub i18n: Rc<I18nService>,
}

#[function_component(PdfPreview)]
pub fn pdf_preview(props: &PdfPreviewProps) -> Html {
    let iframe_ref = use_node_ref();
    let zoom = use_state(|| 100);
    let loading_ctx = use_context::<LoadingContext>().expect("No LoadingContext found");
    let i18n = &props.i18n;

    let onzoomin = {
        let zoom = zoom.clone();
        Callback::from(move |_| {
            zoom.set((*zoom).min(200) + 25);
        })
    };

    let onzoomout = {
        let zoom = zoom.clone();
        Callback::from(move |_| {
            zoom.set((*zoom).max(50) - 25);
        })
    };

    use_effect_with_deps(
        move |(document, iframe_ref, zoom)| {
            if let Some(iframe) = iframe_ref.cast::<HtmlIframeElement>() {
                // Convert PDF content to base64
                let content = base64::encode(&document.content());
                let url = format!("data:application/pdf;base64,{}", content);
                
                // Set iframe src to PDF content
                iframe.set_src(&url);

                // Apply zoom level
                if let Ok(style) = iframe.style() {
                    let _ = style.set_property("transform", &format!("scale({}%)", zoom));
                    let _ = style.set_property("transform-origin", "top left");
                }
            }
            || ()
        },
        (props.document.clone(), iframe_ref.clone(), *zoom),
    );

    html! {
        <div class="component-container pdf-preview" role="region" 
            aria-label={i18n.get_translation("preview.title").unwrap_or_default()}>
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4">
                <div class="flex justify-between items-center mb-4">
                    <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                        {i18n.get_translation("preview.pdf_preview").unwrap_or_default()}
                    </h3>
                    <div class="flex items-center space-x-2">
                        <button 
                            onclick={onzoomout}
                            disabled={*zoom <= 50}
                            class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 disabled:opacity-50"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 12H4"/>
                            </svg>
                        </button>
                        <span class="text-sm text-gray-600 dark:text-gray-400">
                            {format!("{}%", *zoom)}
                        </span>
                        <button 
                            onclick={onzoomin}
                            disabled={*zoom >= 200}
                            class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 disabled:opacity-50"
                        >
                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                            </svg>
                        </button>
                    </div>
                </div>

                <div class="relative overflow-auto" style="height: 600px;">
                    <iframe
                        ref={iframe_ref}
                        class="w-full h-full border-0 rounded bg-white"
                        title={i18n.get_translation("preview.pdf_preview").unwrap_or_default()}
                    />
                </div>

                <div class="mt-4 flex justify-between items-center">
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        {format!("{} {}", 
                            i18n.get_translation("preview.file").unwrap_or_default(),
                            props.document.name()
                        )}
                    </div>
                    <div class="text-sm text-gray-600 dark:text-gray-400">
                        {format!("{} {:.2} KB", 
                            i18n.get_translation("preview.size").unwrap_or_default(),
                            props.document.size() as f64 / 1024.0
                        )}
                    </div>
                </div>

                if props.document.is_encrypted() {
                    <div class="mt-2 text-sm text-green-600 dark:text-green-400 flex items-center">
                        <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" 
                                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                        </svg>
                        {i18n.get_translation("preview.encrypted").unwrap_or_default()}
                    </div>
                }
            </div>
        </div>
    }
}
