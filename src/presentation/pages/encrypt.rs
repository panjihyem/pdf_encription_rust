use yew::prelude::*;
use crate::domain::entities::PdfDocument;
use crate::application::use_cases::encrypt_pdf_use_case::EncryptPdfUseCase;
use crate::presentation::components::{FileUpload, PdfPreview, EncryptionControls};
use crate::presentation::context::LoadingContext;

pub struct EncryptPage {
    pdf_document: Option<PdfDocument>,
    error: Option<String>,
}

pub enum EncryptMsg {
    SetDocument(PdfDocument),
    SetError(String),
    ClearError,
    EncryptDocument(String),
    Reset,
}

impl Component for EncryptPage {
    type Message = EncryptMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            pdf_document: None,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EncryptMsg::SetDocument(doc) => {
                self.pdf_document = Some(doc);
                self.error = None;
                true
            }
            EncryptMsg::SetError(error) => {
                self.error = Some(error);
                true
            }
            EncryptMsg::ClearError => {
                self.error = None;
                true
            }
            EncryptMsg::EncryptDocument(password) => {
                if let Some(doc) = self.pdf_document.clone() {
                    let loading = ctx.link()
                        .context::<LoadingContext>()
                        .expect("No LoadingContext found");
                    
                    loading.set_loading.emit((true, Some("Encrypting PDF...".to_string())));
                    
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match EncryptPdfUseCase::new().encrypt(doc, &password).await {
                            Ok(encrypted_doc) => {
                                link.send_message(EncryptMsg::SetDocument(encrypted_doc));
                                loading.set_loading.emit((false, None));
                            }
                            Err(e) => {
                                link.send_message(EncryptMsg::SetError(e.to_string()));
                                loading.set_loading.emit((false, None));
                            }
                        }
                    });
                }
                false
            }
            EncryptMsg::Reset => {
                self.pdf_document = None;
                self.error = None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onfile = ctx.link().callback(EncryptMsg::SetDocument);
        let onencrypt = ctx.link().callback(EncryptMsg::EncryptDocument);
        let onreset = ctx.link().callback(|_| EncryptMsg::Reset);

        html! {
            <div class="container mx-auto px-4 py-8">
                <h1 class="text-3xl font-bold mb-8 text-center text-gray-900 dark:text-white">
                    {"Encrypt PDF"}
                </h1>

                if let Some(error) = &self.error {
                    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded relative mb-4" role="alert">
                        <span class="block sm:inline">{error}</span>
                        <button 
                            class="absolute top-0 bottom-0 right-0 px-4 py-3"
                            onclick={ctx.link().callback(|_| EncryptMsg::ClearError)}
                        >
                            <span class="sr-only">{"Close"}</span>
                            <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                            </svg>
                        </button>
                    </div>
                }

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div>
                        <FileUpload on_file_selected={onfile} />
                        if let Some(doc) = &self.pdf_document {
                            <div class="mt-8">
                                <EncryptionControls
                                    document={doc.clone()}
                                    on_encrypt={onencrypt}
                                    on_reset={onreset}
                                />
                            </div>
                        }
                    </div>
                    <div>
                        if let Some(doc) = &self.pdf_document {
                            <PdfPreview document={doc.clone()} />
                        }
                    </div>
                </div>
            </div>
        }
    }
}
