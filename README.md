# PDF Encryption Application

A secure, client-side PDF encryption and decryption application built with Rust, WebAssembly, and Yew.

## Features

- 🔒 Client-side PDF encryption/decryption
- 🌐 Multi-language support (EN/ID)
- 🎨 Light/Dark theme
- 📱 Responsive design
- 🔌 Offline support
- 👀 PDF preview
- 📊 Progress tracking

## Tech Stack

- **Frontend Framework**: Yew (Rust-based)
- **Backend**: WebAssembly (WASM)
- **Core Language**: Rust
- **Styling**: Tailwind CSS
- **PDF Processing**: pdf-rs/lopdf
- **Encryption**: ring/RustCrypto
- **i18n**: yew-i18n
- **Storage**: LocalStorage/IndexedDB

## Project Structure

```
pdf-encrypt-wasm/
├── Cargo.toml
├── index.html
├── src/
│   ├── domain/
│   │   ├── entities/
│   │   │   ├── pdf.rs
│   │   │   └── user_settings.rs
│   │   └── repositories/
│   │       ├── pdf_repository.rs
│   │       └── settings_repository.rs
│   │
│   ├── application/
│   │   ├── use_cases/
│   │   │   ├── encrypt_pdf.rs
│   │   │   ├── decrypt_pdf.rs
│   │   │   └── manage_settings.rs
│   │   └── ports/
│   │       ├── pdf_service_port.rs
│   │       └── storage_port.rs
│   │
│   ├── infrastructure/
│   │   ├── wasm/
│   │   │   ├── pdf_processor.rs
│   │   │   └── crypto_engine.rs
│   │   ├── storage/
│   │   │   └── local_storage.rs
│   │   └── i18n/
│   │       ├── en.json
│   │       └── id.json
│   │
│   ├── presentation/
│   │   ├── components/
│   │   │   ├── file_upload.rs
│   │   │   ├── pdf_preview.rs
│   │   │   ├── language_selector.rs
│   │   │   └── theme_toggle.rs
│   │   ├── pages/
│   │   │   ├── home.rs
│   │   │   └── settings.rs
│   │   └── state/
│   │       └── app_state.rs
│   │
│   ├── lib.rs
│   └── main.rs
│
├── styles/
│   ├── main.css
│   └── themes.css
│
└── README.md
```

## Implementation Phases

### Phase 1: Project Setup
- Initialize Rust + Wasm project
- Setup Yew framework
- Configure Tailwind CSS
- Setup project structure

### Phase 2: Domain Implementation
- PDF Entity implementation
- Settings Entity implementation
- Repository interfaces

### Phase 3: Application Layer
- PDF encryption/decryption use cases
- Settings management
- Service interfaces implementation

### Phase 4: Infrastructure
- WASM module implementation
- Storage implementation
- i18n setup

### Phase 5: UI Implementation
- Component development
- Page layouts
- Theme system
- Responsive design

## Dependencies

```toml
[dependencies]
yew = "0.21"
wasm-bindgen = "0.2"
web-sys = "0.3"
js-sys = "0.3"
wasm-logger = "0.2"
log = "0.4"
pdf = "0.8"
ring = "0.17"
yew-i18n = "0.1"
gloo-storage = "0.3"
```

## UI/UX Specifications

### Theme Colors

#### Light Theme
- Primary: #2563eb (Blue)
- Background: #ffffff
- Text: #1f2937
- Surface: #f3f4f6
- Accent: #3b82f6

#### Dark Theme
- Primary: #3b82f6
- Background: #111827
- Text: #f3f4f6
- Surface: #1f2937
- Accent: #60a5fa

### Responsive Breakpoints
- Mobile: < 640px
- Tablet: 640px - 1024px
- Desktop: > 1024px

### Accessibility Features
- ARIA labels
- Keyboard navigation
- High contrast mode
- Screen reader support
- Focus indicators

## Development Guidelines

### Error Handling
- Custom error types
- User-friendly messages
- Error boundaries

### Testing Strategy
- Unit tests per module
- Integration tests
- WASM specific tests

### Performance Optimization
- Lazy loading
- Web Workers
- Memory management
- File size optimization

## Getting Started

1. Install Rust and wasm-pack
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
```

2. Clone the repository
```bash
git clone <repository-url>
cd pdf-encrypt-wasm
```

3. Install dependencies
```bash
cargo build
```

4. Run development server
```bash
trunk serve
```

5. Build for production
```bash
trunk build --release
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
