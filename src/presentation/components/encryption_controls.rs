use web_sys::HtmlInputElement;
use yew::prelude::*;
use crate::domain::entities::{EncryptionStatus, PdfDocument};
use crate::infrastructure::I18nService;
use std::rc::Rc;

pub enum EncryptionMsg {
    SetPassword(String),
    TogglePasswordVisibility,
    Encrypt,
    Decrypt,
    Clear,
    HandleKeyPress(KeyboardEvent),
    SetConfirmPassword(String),
    ValidatePassword,
}

#[derive(Properties, PartialEq)]
pub struct EncryptionControlsProps {
    pub document: PdfDocument,
    pub i18n: Rc<I18nService>,
    pub on_encrypt: Callback<String>,
    pub on_decrypt: Callback<String>,
    pub on_clear: Callback<()>,
    pub on_reset: Callback<()>,
}

pub struct EncryptionControls {
    password: String,
    show_password: bool,
    confirm_password: String,
    error: Option<String>,
}

impl Component for EncryptionControls {
    type Message = EncryptionMsg;
    type Properties = EncryptionControlsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            password: String::new(),
            show_password: false,
            confirm_password: String::new(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            EncryptionMsg::SetPassword(value) => {
                self.password = value;
                self.error = None;
                true
            }
            EncryptionMsg::TogglePasswordVisibility => {
                self.show_password = !self.show_password;
                true
            }
            EncryptionMsg::Encrypt => {
                if self.password != "" && self.password == self.confirm_password {
                    ctx.props().on_encrypt.emit(self.password.clone());
                } else {
                    self.error = Some("Passwords do not match".to_string());
                }
                true
            }
            EncryptionMsg::Decrypt => {
                if self.password != "" {
                    ctx.props().on_decrypt.emit(self.password.clone());
                }
                true
            }
            EncryptionMsg::Clear => {
                self.password = String::new();
                ctx.props().on_clear.emit(());
                true
            }
            EncryptionMsg::HandleKeyPress(event) => {
                if event.key() == "Enter" {
                    if let Some(doc) = &ctx.props().document {
                        if doc.encryption_status == EncryptionStatus::Encrypted {
                            ctx.props().on_decrypt.emit(self.password.clone());
                        } else {
                            ctx.props().on_encrypt.emit(self.password.clone());
                        }
                    }
                }
                false
            }
            EncryptionMsg::SetConfirmPassword(value) => {
                self.confirm_password = value;
                self.error = None;
                true
            }
            EncryptionMsg::ValidatePassword => {
                if self.password.len() < 8 {
                    self.error = Some("Password must be at least 8 characters long".to_string());
                } else if self.password != self.confirm_password {
                    self.error = Some("Passwords do not match".to_string());
                } else {
                    self.error = None;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().callback(|e: KeyboardEvent| EncryptionMsg::HandleKeyPress(e));
        let i18n = &ctx.props().i18n;

        let password_input_id = "password-input";
        let show_password_id = "show-password";
        let encrypt_btn_id = "encrypt-btn";
        let decrypt_btn_id = "decrypt-btn";
        let clear_btn_id = "clear-btn";

        html! {
            <div class="encryption-controls" role="region" aria-label={i18n.get_translation("encryption.controls.title").unwrap_or_default()}>
                <div class="password-input-group">
                    <label for={password_input_id} class="sr-only">
                        {i18n.get_translation("encryption.password.label").unwrap_or_default()}
                    </label>
                    <input
                        id={password_input_id}
                        type={if self.show_password { "text" } else { "password" }}
                        value={self.password.clone()}
                        onchange={ctx.link().callback(|e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            EncryptionMsg::SetPassword(input.value())
                        })}
                        onkeypress={onkeypress}
                        placeholder={i18n.get_translation("encryption.password.placeholder").unwrap_or_default()}
                        aria-required="true"
                        aria-describedby="password-description"
                        minlength="8"
                    />
                    <div id="password-description" class="sr-only">
                        {i18n.get_translation("encryption.password.description").unwrap_or_default()}
                    </div>
                    
                    <div class="password-visibility">
                        <input
                            type="checkbox"
                            id={show_password_id}
                            checked={self.show_password}
                            onchange={ctx.link().callback(|_| EncryptionMsg::TogglePasswordVisibility)}
                            aria-label={i18n.get_translation("encryption.password.show").unwrap_or_default()}
                        />
                        <label for={show_password_id}>
                            {i18n.get_translation("encryption.password.show").unwrap_or_default()}
                        </label>
                    </div>
                </div>

                <div class="password-input-group">
                    <label for="confirm-password-input" class="sr-only">
                        {i18n.get_translation("encryption.confirm_password.label").unwrap_or_default()}
                    </label>
                    <input
                        id="confirm-password-input"
                        type={if self.show_password { "text" } else { "password" }}
                        value={self.confirm_password.clone()}
                        onchange={ctx.link().callback(|e: Event| {
                            let input: HtmlInputElement = e.target_unchecked_into();
                            EncryptionMsg::SetConfirmPassword(input.value())
                        })}
                        onkeypress={onkeypress}
                        placeholder={i18n.get_translation("encryption.confirm_password.placeholder").unwrap_or_default()}
                        aria-required="true"
                        aria-describedby="confirm-password-description"
                        minlength="8"
                    />
                    <div id="confirm-password-description" class="sr-only">
                        {i18n.get_translation("encryption.confirm_password.description").unwrap_or_default()}
                    </div>
                </div>

                if let Some(err) = &self.error {
                    <div class="text-red-500 text-sm">{err}</div>
                }

                <div class="button-group" role="group" aria-label={i18n.get_translation("encryption.actions.group").unwrap_or_default()}>
                    if let Some(doc) = &ctx.props().document {
                        if doc.encryption_status == EncryptionStatus::Encrypted {
                            <button
                                id={decrypt_btn_id}
                                onclick={ctx.link().callback(|_| EncryptionMsg::Decrypt)}
                                disabled={self.password.is_empty()}
                                aria-disabled={self.password.is_empty()}
                            >
                                {i18n.get_translation("encryption.actions.decrypt").unwrap_or_default()}
                            </button>
                        } else {
                            <button
                                id={encrypt_btn_id}
                                onclick={ctx.link().callback(|_| EncryptionMsg::Encrypt)}
                                disabled={self.password.is_empty() || self.password != self.confirm_password}
                                aria-disabled={self.password.is_empty() || self.password != self.confirm_password}
                            >
                                {i18n.get_translation("encryption.actions.encrypt").unwrap_or_default()}
                            </button>
                        }
                        <button
                            id={clear_btn_id}
                            onclick={ctx.link().callback(|_| EncryptionMsg::Clear)}
                            class="secondary"
                            aria-label={i18n.get_translation("encryption.actions.clear").unwrap_or_default()}
                        >
                            {i18n.get_translation("encryption.actions.clear").unwrap_or_default()}
                        </button>
                    }
                </div>
            </div>
        }
    }
}
