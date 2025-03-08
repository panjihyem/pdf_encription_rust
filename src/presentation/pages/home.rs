use yew::prelude::*;
use yew_router::prelude::*;
use crate::presentation::app::Route;

pub struct HomePage;

impl Component for HomePage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container mx-auto px-4 py-8">
                <div class="text-center">
                    <h1 class="text-4xl font-bold mb-4 text-gray-900 dark:text-white">
                        {"PDF Encryption App"}
                    </h1>
                    <p class="text-xl text-gray-600 dark:text-gray-300 mb-8">
                        {"Secure your PDF documents with client-side encryption"}
                    </p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                    <Link<Route> to={Route::Encrypt} classes="block">
                        <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg hover:shadow-xl transition-shadow">
                            <h2 class="text-2xl font-semibold mb-4 text-gray-900 dark:text-white">
                                {"Encrypt PDF"}
                            </h2>
                            <p class="text-gray-600 dark:text-gray-300">
                                {"Protect your PDF files with AES-256-GCM encryption. Add password protection to your sensitive documents."}
                            </p>
                        </div>
                    </Link<Route>>

                    <Link<Route> to={Route::Decrypt} classes="block">
                        <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-lg hover:shadow-xl transition-shadow">
                            <h2 class="text-2xl font-semibold mb-4 text-gray-900 dark:text-white">
                                {"Decrypt PDF"}
                            </h2>
                            <p class="text-gray-600 dark:text-gray-300">
                                {"Access your encrypted PDF files with the correct password. All processing happens in your browser."}
                            </p>
                        </div>
                    </Link<Route>>
                </div>

                <div class="mt-12 text-center">
                    <h3 class="text-2xl font-semibold mb-6 text-gray-900 dark:text-white">
                        {"Key Features"}
                    </h3>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto">
                        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
                            <h4 class="font-semibold mb-2 text-gray-900 dark:text-white">
                                {"Client-side Processing"}
                            </h4>
                            <p class="text-gray-600 dark:text-gray-300">
                                {"All encryption happens in your browser for maximum security"}
                            </p>
                        </div>
                        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
                            <h4 class="font-semibold mb-2 text-gray-900 dark:text-white">
                                {"AES-256-GCM"}
                            </h4>
                            <p class="text-gray-600 dark:text-gray-300">
                                {"Military-grade encryption for your documents"}
                            </p>
                        </div>
                        <div class="bg-white dark:bg-gray-800 p-4 rounded-lg shadow">
                            <h4 class="font-semibold mb-2 text-gray-900 dark:text-white">
                                {"Offline Support"}
                            </h4>
                            <p class="text-gray-600 dark:text-gray-300">
                                {"Works without internet connection"}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
