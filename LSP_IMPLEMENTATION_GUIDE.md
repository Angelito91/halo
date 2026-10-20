# Language Server Protocol (LSP) Implementation Guide para Halo

> ⚠️ **Diseño futuro — no implementado.** No existe `src/lsp` ni dependencias
> LSP en `Cargo.toml`. Es una guía de implementación a futuro, no un manual de
> algo que ya funcione.

## 📚 Tabla de Contenidos
1. [Introducción](#introducción)
2. [Arquitectura General](#arquitectura-general)
3. [Plan de Implementación](#plan-de-implementación)
4. [Paso a Paso](#paso-a-paso)
5. [Ejemplos de Código](#ejemplos-de-código)
6. [Configuración de Editors](#configuración-de-editors)

---

## Introducción

Un **Language Server Protocol (LSP)** es un protocolo estándar que permite que editores y IDEs (VS Code, Neovim, Emacs, etc.) se comuniquen con un servidor de lenguaje para ofrecer:

- **Autocompletado** (IntelliSense)
- **Ir a definición** (Go to Definition)
- **Búsqueda de referencias** (Find References)
- **Diagnósticos** (Errores y advertencias en tiempo real)
- **Formato de código** (Code Formatting)
- **Rename de variables** (Rename Symbol)
- **Hover information** (Info de tipos/funciones)
- **Document symbols** (Navegación en archivo)

Para Halo, implementaremos un LSP usando la biblioteca **`lsp-types`** que comunica sobre **stdio o TCP**.

---

## Arquitectura General

```
┌─────────────────────────────────────────────────────────────────┐
│                    VS Code / Neovim / Emacs                     │
└─────────────────────────────────────────────────────────────────┘
                              ↕ (JSON-RPC)
┌─────────────────────────────────────────────────────────────────┐
│                    LSP Client Extension                          │
│  (vscode-halo, nvim-halo, etc.)                                 │
└─────────────────────────────────────────────────────────────────┘
                              ↕ (stdio)
┌─────────────────────────────────────────────────────────────────┐
│                  halo-lsp (nuestro servidor)                    │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ LSP Protocol Handler                                       │ │
│  │ - initialize                                              │ │
│  │ - textDocument/didOpen                                   │ │
│  │ - textDocument/didChange                                 │ │
│  │ - textDocument/completion                                │ │
│  │ - textDocument/definition                                │ │
│  │ - textDocument/references                                │ │
│  │ - textDocument/hover                                     │ │
│  │ - textDocument/formatting                                │ │
│  └────────────────────────────────────────────────────────────┘ │
│                              ↕                                   │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │ Analysis Engine                                            │ │
│  │ - Lexer                                                   │ │
│  │ - Parser                                                 │ │
│  │ - Type Checker                                           │ │
│  │ - Symbol Table                                           │ │
│  │ - Diagnostics                                            │ │
│  └────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                              ↓
                         File System
```

---

## Plan de Implementación

### Fase 1: Setup Inicial (0.5-1 hora)
- [ ] Crear nuevo binary `halo-lsp` en `src/bin/lsp.rs`
- [ ] Agregar dependencias: `lsp-server`, `lsp-types`, `serde_json`
- [ ] Crear módulo `src/lsp/mod.rs`

### Fase 2: Protocol Foundation (1-2 horas)
- [ ] Implementar loop de mensaje JSON-RPC
- [ ] Manejar `initialize` request
- [ ] Implementar `initialized` notification
- [ ] Setup de logging

### Fase 3: Document Management (1-2 horas)
- [ ] Manejar `textDocument/didOpen`
- [ ] Manejar `textDocument/didChange`
- [ ] Manejar `textDocument/didClose`
- [ ] Mantener caché de documentos en memoria

### Fase 4: Basic Analysis (2-3 horas)
- [ ] Implementar `textDocument/publish diagnostics`
- [ ] Análisis léxico en tiempo real
- [ ] Análisis sintáctico en tiempo real
- [ ] Reportar errores/advertencias

### Fase 5: Completion (2-3 horas)
- [ ] Implementar `textDocument/completion`
- [ ] Extraer símbolos (variables, funciones)
- [ ] Generar suggestions contextuales

### Fase 6: Navigation (2-3 horas)
- [ ] Implementar `textDocument/definition`
- [ ] Implementar `textDocument/references`
- [ ] Symbol table completo

### Fase 7: Additional Features (1-2 horas)
- [ ] Hover information
- [ ] Format document
- [ ] Rename symbol
- [ ] Document symbols

---

## Paso a Paso

### Paso 1: Configurar Cargo.toml

```toml
[package]
name = "halo"
version = "0.2.0"

[[bin]]
name = "halo"
path = "src/bin/main.rs"

[[bin]]
name = "halo-lsp"
path = "src/bin/lsp.rs"

[dependencies]
# Existentes...
serde_json = "1.0"
lsp-server = "0.7"
lsp-types = "0.95"
```

### Paso 2: Crear estructura base del LSP

Crear `src/lsp/mod.rs`:

```rust
// The Halo Programming Language - LSP Server
// Version: 0.2.0

pub mod server;
pub mod analysis;
pub mod diagnostics;
pub mod completion;
pub mod navigation;

use lsp_types::*;
use std::collections::HashMap;

/// Estado global del servidor LSP
pub struct HaloLspServer {
    /// Documentos abiertos: uri -> contenido
    pub documents: HashMap<String, String>,
    
    /// Símbolos por documento para análisis rápido
    pub symbols: HashMap<String, Vec<Symbol>>,
    
    /// Capacidades del cliente
    pub client_capabilities: Option<ClientCapabilities>,
    
    /// Si el servidor fue inicializado
    pub initialized: bool,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub range: Range,
}

impl HaloLspServer {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
            symbols: HashMap::new(),
            client_capabilities: None,
            initialized: false,
        }
    }
}
```

### Paso 3: Crear binary `src/bin/lsp.rs`

```rust
use lsp_server::{Connection, Message, Request, Response};
use lsp_types::*;
use serde_json::json;
use std::error::Error;

mod lsp {
    pub use halo::lsp::*;
}

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Starting Halo LSP Server...");
    
    // Crear conexión stdin/stdout
    let (connection, io_threads) = Connection::stdio();
    
    // Crear servidor
    let mut server = lsp::HaloLspServer::new();
    
    // Loop principal de mensajes
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    break;
                }
                
                let response = handle_request(&mut server, &connection, req);
                if let Some(resp) = response {
                    connection.sender.send(resp.into())?;
                }
            }
            
            Message::Notification(notif) => {
                handle_notification(&mut server, &connection, notif)?;
            }
            
            Message::Response(resp) => {
                eprintln!("Unexpected response: {:?}", resp);
            }
        }
    }
    
    io_threads.join()?;
    eprintln!("Halo LSP Server stopped");
    Ok(())
}

fn handle_request(
    server: &mut lsp::HaloLspServer,
    _connection: &Connection,
    req: Request,
) -> Option<Response> {
    match req.method.as_str() {
        "initialize" => {
            let params = serde_json::from_value::<InitializeParams>(req.params).ok()?;
            server.client_capabilities = params.capabilities;
            server.initialized = true;
            
            let capabilities = ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    ..Default::default()
                }),
                definition_provider: Some(OneOf::Left(true)),
                references_provider: Some(OneOf::Left(true)),
                hover_provider: Some(HoverServerCapabilities::Simple(true)),
                document_symbol_provider: Some(OneOf::Left(true)),
                formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            };
            
            Some(Response::new_ok(
                req.id,
                InitializeResult {
                    capabilities,
                    server_info: Some(ServerInfo {
                        name: "Halo".to_string(),
                        version: Some("0.2.0".to_string()),
                    }),
                    ..Default::default()
                },
            ))
        }
        
        "textDocument/completion" => {
            let params = serde_json::from_value::<CompletionParams>(req.params).ok()?;
            let uri = params.text_document_position.text_document.uri.to_string();
            
            if let Some(doc) = server.documents.get(&uri) {
                let completions = lsp::completion::get_completions(doc, &params);
                Some(Response::new_ok(req.id, completions))
            } else {
                Some(Response::new_err(
                    req.id,
                    lsp_server::ErrorCode::InvalidRequest as i32,
                    "Document not found".to_string(),
                ))
            }
        }
        
        "textDocument/definition" => {
            let params = serde_json::from_value::<GotoDefinitionParams>(req.params).ok()?;
            let uri = params.text_document_position_params.text_document.uri.to_string();
            
            if let Some(def) = lsp::navigation::find_definition(
                server,
                &uri,
                &params.text_document_position_params.position,
            ) {
                Some(Response::new_ok(req.id, def))
            } else {
                Some(Response::new_ok(req.id, json!(null)))
            }
        }
        
        "textDocument/hover" => {
            let params = serde_json::from_value::<HoverParams>(req.params).ok()?;
            let uri = params.text_document_position_params.text_document.uri.to_string();
            
            if let Some(hover) = lsp::navigation::hover_info(
                server,
                &uri,
                &params.text_document_position_params.position,
            ) {
                Some(Response::new_ok(req.id, hover))
            } else {
                Some(Response::new_ok(req.id, json!(null)))
            }
        }
        
        _ => Some(Response::new_err(
            req.id,
            lsp_server::ErrorCode::MethodNotFound as i32,
            format!("Method not found: {}", req.method),
        )),
    }
}

fn handle_notification(
    server: &mut lsp::HaloLspServer,
    connection: &Connection,
    notif: lsp_server::Notification,
) -> Result<(), Box<dyn Error>> {
    match notif.method.as_str() {
        "initialized" => {
            eprintln!("Client initialized");
        }
        
        "textDocument/didOpen" => {
            let params = serde_json::from_value::<DidOpenTextDocumentParams>(notif.params)?;
            let uri = params.text_document.uri.to_string();
            let text = params.text_document.text;
            
            server.documents.insert(uri.clone(), text.clone());
            
            // Analizar y enviar diagnósticos
            let diags = lsp::diagnostics::analyze(&text);
            connection.sender.send(Message::Notification(
                lsp_server::Notification {
                    method: "textDocument/publishDiagnostics".to_string(),
                    params: json!({
                        "uri": uri,
                        "diagnostics": diags,
                    }),
                },
            ))?;
        }
        
        "textDocument/didChange" => {
            let params = serde_json::from_value::<DidChangeTextDocumentParams>(notif.params)?;
            let uri = params.text_document.uri.to_string();
            
            if let Some(change) = params.content_changes.first() {
                if let Some(text) = &change.text {
                    server.documents.insert(uri.clone(), text.clone());
                    
                    // Reanálizar
                    let diags = lsp::diagnostics::analyze(text);
                    connection.sender.send(Message::Notification(
                        lsp_server::Notification {
                            method: "textDocument/publishDiagnostics".to_string(),
                            params: json!({
                                "uri": uri,
                                "diagnostics": diags,
                            }),
                        },
                    ))?;
                }
            }
        }
        
        "textDocument/didClose" => {
            let params = serde_json::from_value::<DidCloseTextDocumentParams>(notif.params)?;
            let uri = params.text_document.uri.to_string();
            server.documents.remove(&uri);
            server.symbols.remove(&uri);
        }
        
        _ => {}
    }
    
    Ok(())
}
```

### Paso 4: Módulo de Diagnósticos

Crear `src/lsp/diagnostics.rs`:

```rust
use lsp_types::*;
use crate::lexer::{Lexer, TokenKind};
use crate::parser::Parser;

pub fn analyze(text: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    
    // Análisis léxico
    let mut lexer = Lexer::new(text.to_string());
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token();
        let is_eof = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if is_eof {
            break;
        }
    }
    
    // Análisis sintáctico
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(_) => {
            // Sin errores
        }
        Err(errors) => {
            for error in errors {
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 1,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: error,
                    ..Default::default()
                });
            }
        }
    }
    
    diagnostics
}
```

### Paso 5: Módulo de Completion

Crear `src/lsp/completion.rs`:

```rust
use lsp_types::*;

pub fn get_completions(doc: &str, _params: &CompletionParams) -> Vec<CompletionItem> {
    let mut items = Vec::new();
    
    // Completions predefinidas
    let keywords = vec![
        ("if", "Conditional statement", CompletionItemKind::KEYWORD),
        ("while", "Loop statement", CompletionItemKind::KEYWORD),
        ("fn", "Function definition", CompletionItemKind::KEYWORD),
        ("return", "Return statement", CompletionItemKind::KEYWORD),
        ("break", "Break loop", CompletionItemKind::KEYWORD),
        ("continue", "Continue loop", CompletionItemKind::KEYWORD),
    ];
    
    for (label, detail, kind) in keywords {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(kind),
            detail: Some(detail.to_string()),
            insert_text: Some(label.to_string()),
            ..Default::default()
        });
    }
    
    // Funciones built-in
    let builtins = vec![
        ("print", "Print to output", CompletionItemKind::FUNCTION),
        ("len", "Get length", CompletionItemKind::FUNCTION),
        ("str", "Convert to string", CompletionItemKind::FUNCTION),
        ("int", "Convert to integer", CompletionItemKind::FUNCTION),
        ("float", "Convert to float", CompletionItemKind::FUNCTION),
    ];
    
    for (label, detail, kind) in builtins {
        items.push(CompletionItem {
            label: label.to_string(),
            kind: Some(kind),
            detail: Some(detail.to_string()),
            insert_text: Some(format!("{}()", label)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }
    
    // Extraer símbolos del documento actual
    let symbols = extract_symbols(doc);
    for symbol in symbols {
        items.push(CompletionItem {
            label: symbol.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            insert_text: Some(symbol),
            ..Default::default()
        });
    }
    
    items
}

fn extract_symbols(doc: &str) -> Vec<String> {
    let mut symbols = Vec::new();
    
    // Buscar líneas con "x =" para extraer nombres de variables
    for line in doc.lines() {
        if let Some(eq_pos) = line.find('=') {
            let var_name = line[..eq_pos].trim();
            if !var_name.is_empty() && var_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                symbols.push(var_name.to_string());
            }
        }
    }
    
    symbols
}
```

### Paso 6: Módulo de Navegación

Crear `src/lsp/navigation.rs`:

```rust
use lsp_types::*;
use crate::lsp::HaloLspServer;

pub fn find_definition(
    _server: &HaloLspServer,
    _uri: &str,
    _position: &Position,
) -> Option<GotoDefinitionResponse> {
    // TODO: Implementar búsqueda de definición
    // Requiere:
    // 1. Extraer token en position
    // 2. Buscar en symbol table
    // 3. Retornar Location de definición
    None
}

pub fn hover_info(
    _server: &HaloLspServer,
    _uri: &str,
    _position: &Position,
) -> Option<Hover> {
    // TODO: Implementar hover
    // Retorna tipo e información del símbolo
    None
}
```

---

## Configuración de Editors

### VS Code

1. Crear archivo `.vscode/launch.json`:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Halo LSP",
      "type": "extensionHost",
      "request": "launch",
      "args": ["--extensionDevelopmentPath=${workspaceFolder}"],
      "outFiles": ["${workspaceFolder}/out/**/*.js"],
      "preLaunchTask": "npm"
    }
  ]
}
```

2. Crear extensión en `vscode-halo/`:

```typescript
// package.json
{
  "name": "vscode-halo",
  "displayName": "Halo Language",
  "version": "0.2.0",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Programming Languages"],
  "activationEvents": ["onLanguage:halo"],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [
      {
        "id": "halo",
        "aliases": ["Halo"],
        "extensions": [".halo"],
        "configuration": "./language-configuration.json"
      }
    ],
    "grammars": [
      {
        "language": "halo",
        "scopeName": "source.halo",
        "path": "./syntaxes/halo.tmLanguage.json"
      }
    ]
  }
}
```

```typescript
// extension.ts
import * as vscode from 'vscode';
import { LanguageClient, ServerOptions, LanguageClientOptions } from 'vscode-languageclient/node';

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
  const serverOptions: ServerOptions = {
    command: 'halo-lsp',
    args: [],
  };

  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ scheme: 'file', language: 'halo' }],
  };

  client = new LanguageClient(
    'haloLanguageServer',
    'Halo Language Server',
    serverOptions,
    clientOptions
  );

  client.start();
}

export function deactivate(): Thenable<void> | undefined {
  if (!client) {
    return undefined;
  }
  return client.stop();
}
```

### Neovim (init.lua)

```lua
local nvim_lsp = require('lspconfig')

nvim_lsp.halo.setup {
  cmd = { 'halo-lsp' },
  on_attach = function(client, bufnr)
    local opts = { noremap=true, silent=true }
    vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gd', '<cmd>lua vim.lsp.buf.definition()<CR>', opts)
    vim.api.nvim_buf_set_keymap(bufnr, 'n', 'K', '<cmd>lua vim.lsp.buf.hover()<CR>', opts)
    vim.api.nvim_buf_set_keymap(bufnr, 'n', 'gi', '<cmd>lua vim.lsp.buf.implementation()<CR>', opts)
    vim.api.nvim_buf_set_keymap(bufnr, 'n', '<C-k>', '<cmd>lua vim.lsp.buf.signature_help()<CR>', opts)
  end,
}
```

---

## Compilación y Ejecución

### Compilar el LSP:

```bash
cargo build --bin halo-lsp --release
```

### Ejecutar en modo debug:

```bash
RUST_LOG=debug cargo run --bin halo-lsp
```

### Instalar en sistema:

```bash
cargo install --path . --bin halo-lsp
# Ahora disponible como: halo-lsp
```

---

## Estructura Recomendada Final

```
src/
├── bin/
│   ├── main.rs          (CLI existente)
│   └── lsp.rs           (Nuevo LSP server)
├── lsp/
│   ├── mod.rs           (Estado y tipos)
│   ├── server.rs        (Loop principal)
│   ├── analysis.rs      (Análisis en tiempo real)
│   ├── diagnostics.rs   (Errores y advertencias)
│   ├── completion.rs    (Autocompletado)
│   ├── navigation.rs    (Go to definition, hover)
│   └── formatting.rs    (Code formatting)
├── lexer/
├── parser/
├── compiler/
├── interpreter/
└── ... (resto del proyecto)
```

---

## Próximos Pasos

1. **Análisis Incremental**: Cachear AST para analizar solo cambios
2. **Symbol Table Completo**: Seguimiento de variables y funciones por scope
3. **Type Information**: Integración con type checker
4. **Refactoring**: Renaming, extract function, etc.
5. **Testing**: Tests para cada feature del LSP

---

## Referencias

- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [lsp-server crate](https://docs.rs/lsp-server/)
- [lsp-types crate](https://docs.rs/lsp-types/)
- [VS Code Extension API](https://code.visualstudio.com/api)
