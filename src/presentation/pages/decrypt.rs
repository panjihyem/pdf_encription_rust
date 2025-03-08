use yew::prelude::*;
use web_sys::File;
use crate::domain::entities::PdfDocument;
use crate::domain::error::PdfError;
use crate::application::use_cases::decrypt_pdf_use_case::DecryptPdfUseCase;
use crate::presentation::components::{
    FileUpload, PdfPreview, LoadingIndicator, EncryptionControls
};
use crate::presentation::context::LoadingContext;
use crate::infrastructure::I18nService;

#[derive(Properties, PartialEq)]
pub struct DecryptPageProps {
    pub i18n: I18nService,
}

pub enum DecryptMsg {
    FileSelected(File),
    PasswordEntered(String),
    DecryptFile,
    DecryptionSuccess(PdfDocument),
    DecryptionError(PdfError),
}

pub struct DecryptPage {
    selected_file: Option<File>,
    password: String,
    decrypted_document: Option<PdfDocument>,
    error: Option<PdfError>,
    loading: bool,
}

impl Component for DecryptPage {
    type Message = DecryptMsg;
    type Properties = DecryptPageProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            selected_file: None,
            password: String::new(),
            decrypted_document: None,
            error: None,
            loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            DecryptMsg::FileSelected(file) => {
                self.selected_file = Some(file);
                self.error = None;
                true
            }
            DecryptMsg::PasswordEntered(password) => {
                self.password = password;
                self.error = None;
                true
            }
            DecryptMsg::DecryptFile => {
                if let Some(file) = &self.selected_file {
                    self.loading = true;
                    let password = self.password.clone();
                    
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        match DecryptPdfUseCase::new().decrypt_pdf(file, &password).await {
                            Ok(decrypted) => link.send_message(DecryptMsg::DecryptionSuccess(decrypted)),
                            Err(e) => link.send_message(DecryptMsg::DecryptionError(e)),
                        }
                    });
                }
                true
            }
            DecryptMsg::DecryptionSuccess(doc) => {
                self.decrypted_document = Some(doc);
                self.loading = false;
                self.error = None;
                true
            }
            DecryptMsg::DecryptionError(error) => {
                self.error = Some(error);
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let i18n = &ctx.props().i18n;
        
        let onfile = ctx.link().callback(DecryptMsg::FileSelected);
        let onpassword = ctx.link().callback(DecryptMsg::PasswordEntered);
        let ondecrypt = ctx.link().callback(|_| DecryptMsg::DecryptFile);

        html! {
            <div class="container mx-auto px-4 py-8">
                <div class="max-w-4xl mx-auto">
                    <h1 class="text-3xl font-bold mb-8 text-gray-900 dark:text-white">
                        {i18n.get_translation("decrypt.title").unwrap_or_default()}
                    </h1>

                    <div class="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-6 mb-8">
                        <FileUpload
                            {onfile}
                            accept="application/pdf"
                            i18n={i18n.clone()}
                        />

                        if self.selected_file.is_some() {
                            <div class="mt-6">
                                <EncryptionControls
                                    onpassword={onpassword}
                                    onsubmit={ondecrypt}
                                    mode="decrypt"
                                    disabled={self.loading}
                                    i18n={i18n.clone()}
                                />
                            </div>
                        }

                        if self.loading {
                            <div class="mt-6">
                                <LoadingIndicator 
                                    message={i18n.get_translation("decrypt.processing").unwrap_or_default()}
                                />
                            </div>
                        }

                        if let Some(error) = &self.error {
                            <div class="mt-6 p-4 rounded relative">
                                {self.render_error(error, i18n)}
                            </div>
                        }
                    </div>

                    if let Some(doc) = &self.decrypted_document {
                        <div class="mt-8">
                            <PdfPreview 
                                document={doc.clone()}
                                i18n={i18n.clone()}
                            />
                        </div>
                    }
                </div>
            </div>
        }
    }
}

impl DecryptPage {
    fn render_error(&self, error: &PdfError, i18n: &I18nService) -> Html {
        let (message, style) = match error {
            PdfError::WeakPassword(_) | PdfError::ValidationError(_) => (
                error.to_string(),
                "bg-yellow-100 border border-yellow-400 text-yellow-800"
            ),
            PdfError::DecryptionFailed(_) | PdfError::InvalidPassword(_) => (
                error.to_string(),
                "bg-red-100 border border-red-400 text-red-800"
            ),
            PdfError::FileNotFound(_) | PdfError::InvalidFileFormat(_) => (
                error.to_string(),
                "bg-orange-100 border border-orange-400 text-orange-800"
            ),
            _ => (
                i18n.get_translation("error.unexpected").unwrap_or_else(|| error.to_string()),
                "bg-red-100 border border-red-400 text-red-800"
            ),
        };

        html! {
            <div class={style}>
                <div class="flex items-center">
                    <div class="flex-shrink-0">
                        <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" 
                                d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" 
                                clip-rule="evenodd"
                            />
                        </svg>
                    </div>
                    <div class="ml-3">
                        <p class="text-sm">{message}</p>
                    </div>
                </div>
            </div>
        }
    }
}
