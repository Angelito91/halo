# Arquitectura Técnica del LSP para Halo

> ⚠️ **Diseño futuro — no implementado.** No existe `src/lsp` ni dependencias
> LSP en `Cargo.toml`. Este documento describe el diseño previsto, no código
> que se pueda ejecutar hoy.

## 1. Visión General

Un Language Server sigue la arquitectura cliente-servidor donde:

- **Servidor**: Analiza código, proporciona información
- **Cliente**: Integración con editor (VS Code, Neovim, etc.)
- **Protocolo**: JSON-RPC sobre stdio/TCP

Para Halo, usaremos **stdio** (más simple) inicialmente.

---

## 2. Flujo de Comunicación

### Ejemplo: Usuario escribe `prin|` (cursor donde `|`)

```
1. Editor detecta cambio
   └─> envía textDocument/didChange

2. Servidor recibe:
   {
     "jsonrpc": "2.0",
     "method": "textDocument/didChange",
     "params": {
       "textDocument": { "uri": "file:///home/user/main.halo" },
       "contentChanges": [{ "text": "prin" }]
     }
   }

3. Servidor analiza código
   ├─> Lexer: tokeniza
   ├─> Parser: crea AST
   ├─> Analyzer: extrae símbolos, busca errores
   └─> Storage: cachea en memoria

4. Servidor envía textDocument/publishDiagnostics
   {
     "method": "textDocument/publishDiagnostics",
     "params": {
       "uri": "file:///home/user/main.halo",
       "diagnostics": []  // Sin errores (aún incompleto)
     }
   }

5. Usuario presiona Ctrl+Space (completado)
   └─> envía textDocument/completion

6. Servidor responde con sugerencias:
   {
     "items": [
       {
         "label": "print",
         "kind": 12,  // FUNCTION
         "insertText": "print($0)"
       },
       {
         "label": "println",
         "kind": 12
       }
     ]
   }

7. Editor muestra popup con sugerencias
```

---

## 3. Componentes Principales

### 3.1 LSP Server Core

```rust
pub struct HaloLspServer {
    // Gestión de documentos abiertos
    documents: HashMap<String, DocumentState>,
    
    // Cache de análisis
    analysis_cache: HashMap<String, AnalysisResult>,
    
    // Información del cliente
    client_capabilities: ClientCapabilities,
    
    // Configuración del servidor
    config: ServerConfig,
}

pub struct DocumentState {
    uri: String,
    version: i32,
    text: String,
    
    // Resultados del análisis
    tokens: Vec<Token>,
    ast: Option<Program>,
    diagnostics: Vec<Diagnostic>,
    symbols: Vec<Symbol>,
}

pub struct AnalysisResult {
    // Parsing
    errors: Vec<ParseError>,
    
    // Semantic analysis
    variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, FunctionInfo>,
    
    // Type information
    type_info: HashMap<Position, TypeInfo>,
}
```

### 3.2 Handlers de Protocolo

```rust
impl HaloLspServer {
    // ========== Lifecycle ==========
    
    fn initialize(&mut self, params: InitializeParams) -> InitializeResult {
        // 1. Guardar client_capabilities
        // 2. Retornar server capabilities
        // 3. Setup inicial
    }
    
    fn initialized(&mut self) {
        // Server está listo para recibir mensajes
        // Inicializar recursos si es necesario
    }
    
    fn shutdown(&mut self) {
        // Cleanup
    }
    
    // ========== Document Sync ==========
    
    fn did_open(&mut self, params: DidOpenTextDocumentParams) {
        // 1. Guardar documento
        // 2. Analizar
        // 3. Enviar diagnósticos
    }
    
    fn did_change(&mut self, params: DidChangeTextDocumentParams) {
        // 1. Actualizar contenido
        // 2. Re-analizar (incremental si es posible)
        // 3. Enviar diagnósticos
    }
    
    fn did_close(&mut self, params: DidCloseTextDocumentParams) {
        // 1. Remover de cache
        // 2. Limpiar memoria
    }
    
    // ========== Features ==========
    
    fn completion(&self, params: CompletionParams) -> Vec<CompletionItem> {
        // 1. Extraer contexto en posición
        // 2. Determinar qué completar (variable, función, etc.)
        // 3. Generar sugerencias
    }
    
    fn definition(&self, params: GotoDefinitionParams) -> Location {
        // 1. Extraer símbolo en posición
        // 2. Buscar en symbol table
        // 3. Retornar ubicación de definición
    }
    
    fn references(&self, params: ReferenceParams) -> Vec<Location> {
        // 1. Extraer símbolo
        // 2. Encontrar todos los usos
        // 3. Retornar ubicaciones
    }
    
    fn hover(&self, params: HoverParams) -> Hover {
        // 1. Extraer símbolo
        // 2. Obtener información de tipo
        // 3. Retornar información formateada
    }
}
```

---

## 4. Análisis en Tiempo Real

### 4.1 Pipeline de Análisis

```
Texto → Lexer → Parser → Semantic Analyzer → Results
         ↓        ↓           ↓                  ↓
        Tokens   AST    Symbol Table      Diagnósticos
                         Type Info        Completions
                                         Definitions
```

### 4.2 Incremental Analysis

Para grandes proyectos, no re-analizar todo:

```rust
// Estrategia 1: Análisis completo (simple, para archivos pequeños)
fn analyze_full(text: &str) -> AnalysisResult {
    let tokens = lexer::tokenize(text);
    let ast = parser::parse(tokens)?;
    semantic_analysis(&ast)
}

// Estrategia 2: Análisis incremental (eficiente)
fn analyze_incremental(
    old_state: &DocumentState,
    changes: &[TextDocumentContentChangeEvent],
) -> AnalysisResult {
    let new_text = apply_changes(&old_state.text, changes);
    
    // Solo re-tokenizar/parsear la región afectada
    let affected_range = find_affected_range(&old_state.text, &new_text);
    
    if can_reuse_ast(old_state, &affected_range) {
        // Reusar AST viejo, solo actualizar símbolos locales
        merge_analysis(old_state, new_analysis)
    } else {
        // Necesita reparseo completo
        analyze_full(&new_text)
    }
}
```

---

## 5. Symbol Table y Información de Tipos

### 5.1 Estructura

```rust
pub struct SymbolTable {
    /// Símbolos globales (funciones, variables globales)
    globals: HashMap<String, Symbol>,
    
    /// Scopes anidados (para funciones)
    scopes: Vec<Scope>,
}

pub struct Scope {
    /// Variables locales en este scope
    variables: HashMap<String, VariableInfo>,
    
    /// Scope padre (para búsqueda)
    parent: Option<Box<Scope>>,
}

pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub location: Location,
    pub type_info: Option<TypeInfo>,
    pub documentation: Option<String>,
}

pub enum SymbolKind {
    Variable,
    Function { params: Vec<String>, return_type: Option<String> },
    Builtin,
}

pub struct TypeInfo {
    pub base_type: Type,
    pub is_optional: bool,
}

pub enum Type {
    Number,
    Float,
    String,
    Bool,
    Array(Box<Type>),
    Function { params: Vec<Type>, return_type: Box<Type> },
    Unknown,
}
```

### 5.2 Building the Symbol Table

```rust
impl HaloLspServer {
    fn build_symbol_table(&self, ast: &Program) -> SymbolTable {
        let mut table = SymbolTable::new();
        
        // Pass 1: Recolectar todas las funciones
        for item in &ast.items {
            if let TopLevel::Function { name, params, .. } = item {
                table.add_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Function {
                        params: params.clone(),
                        return_type: None,
                    },
                    location: self.get_location(item),
                    type_info: None,
                    documentation: None,
                });
            }
        }
        
        // Pass 2: Recolectar variables globales
        for item in &ast.items {
            if let TopLevel::GlobalVar { name, .. } = item {
                table.add_symbol(Symbol {
                    name: name.clone(),
                    kind: SymbolKind::Variable,
                    location: self.get_location(item),
                    type_info: None,
                    documentation: None,
                });
            }
        }
        
        table
    }
}
```

---

## 6. Features Detallados

### 6.1 Completion

```rust
pub fn completion(server: &HaloLspServer, params: CompletionParams) -> Vec<CompletionItem> {
    let uri = params.text_document_position.text_document.uri.to_string();
    let position = params.text_document_position.position;
    
    let doc = match server.documents.get(&uri) {
        Some(d) => d,
        None => return vec![],
    };
    
    // Determinar contexto en la posición
    let context = analyze_context(&doc.text, position);
    
    match context {
        Context::Global => {
            // Top-level: keywords + global functions
            keywords_completion()
                .into_iter()
                .chain(global_functions(server))
                .collect()
        }
        
        Context::FunctionBody => {
            // Dentro de función: keywords + variables + functions
            keywords_completion()
                .into_iter()
                .chain(local_variables(server, &uri))
                .chain(global_functions(server))
                .collect()
        }
        
        Context::Expression => {
            // En expresión: variables + functions + methods
            variables_completion(server, &uri)
                .into_iter()
                .chain(functions_completion(server))
                .collect()
        }
    }
}
```

### 6.2 Go to Definition

```rust
pub fn definition(server: &HaloLspServer, params: GotoDefinitionParams) -> Option<Location> {
    let uri = params.text_document_position_params.text_document.uri.to_string();
    let position = params.text_document_position_params.position;
    
    let doc = server.documents.get(&uri)?;
    
    // 1. Extraer token en posición
    let token = find_token_at_position(&doc.text, position)?;
    
    // 2. Buscar en symbol table
    let symbol = server.symbol_table().lookup(&token.text)?;
    
    // 3. Retornar ubicación de definición
    Some(symbol.location)
}
```

### 6.3 Find References

```rust
pub fn references(server: &HaloLspServer, params: ReferenceParams) -> Vec<Location> {
    let uri = params.text_document.uri.to_string();
    let position = params.position;
    
    let doc = match server.documents.get(&uri) {
        Some(d) => d,
        None => return vec![],
    };
    
    // 1. Extraer símbolo
    let token = match find_token_at_position(&doc.text, position) {
        Some(t) => t,
        None => return vec![],
    };
    
    // 2. Buscar todas las referencias
    let mut references = vec![];
    
    // En el documento actual
    for (pos, txt) in find_all_occurrences(&doc.text, &token.text) {
        if is_reference(txt) {
            references.push(Location {
                uri: uri.clone(),
                range: pos,
            });
        }
    }
    
    // En otros documentos (si es necesario)
    for (other_uri, other_doc) in &server.documents {
        if other_uri != &uri {
            for (pos, txt) in find_all_occurrences(&other_doc.text, &token.text) {
                if is_reference(txt) {
                    references.push(Location {
                        uri: other_uri.clone(),
                        range: pos,
                    });
                }
            }
        }
    }
    
    references
}
```

### 6.4 Hover Information

```rust
pub fn hover(server: &HaloLspServer, params: HoverParams) -> Option<Hover> {
    let uri = params.text_document_position_params.text_document.uri.to_string();
    let position = params.text_document_position_params.position;
    
    let doc = server.documents.get(&uri)?;
    let token = find_token_at_position(&doc.text, position)?;
    
    let symbol = server.symbol_table().lookup(&token.text)?;
    
    // Construir información
    let mut contents = String::new();
    
    match &symbol.kind {
        SymbolKind::Variable => {
            contents.push_str(&format!("**Variable**: {}\n", symbol.name));
            if let Some(type_info) = &symbol.type_info {
                contents.push_str(&format!("**Type**: {}\n", type_info.base_type));
            }
        }
        
        SymbolKind::Function { params, return_type } => {
            contents.push_str(&format!("**Function**: {}\n", symbol.name));
            contents.push_str(&format!("**Parameters**: {}\n", params.join(", ")));
            if let Some(ret) = return_type {
                contents.push_str(&format!("**Returns**: {}\n", ret));
            }
        }
        
        SymbolKind::Builtin => {
            contents.push_str(&format!("**Built-in**: {}\n", symbol.name));
        }
    }
    
    if let Some(docs) = &symbol.documentation {
        contents.push_str("\n---\n");
        contents.push_str(docs);
    }
    
    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: contents,
        }),
        range: None,
    })
}
```

---

## 7. Optimizaciones

### 7.1 Caching

```rust
pub struct AnalysisCache {
    /// Cachea análisis por documento
    cache: HashMap<String, CacheEntry>,
    
    /// Máximo tamaño de cache (bytes)
    max_size: usize,
    
    /// Tamaño actual
    current_size: usize,
}

struct CacheEntry {
    analysis: AnalysisResult,
    timestamp: Instant,
    size: usize,
}

impl AnalysisCache {
    fn get(&self, uri: &str) -> Option<&AnalysisResult> {
        self.cache.get(uri).map(|e| &e.analysis)
    }
    
    fn set(&mut self, uri: String, analysis: AnalysisResult) {
        let size = std::mem::size_of_val(&analysis);
        
        // Evictar si es necesario
        while self.current_size + size > self.max_size && !self.cache.is_empty() {
            let oldest = self.cache.values()
                .min_by_key(|e| e.timestamp)
                .map(|e| e.size)
                .unwrap_or(0);
            self.current_size = self.current_size.saturating_sub(oldest);
        }
        
        self.cache.insert(uri, CacheEntry {
            analysis,
            timestamp: Instant::now(),
            size,
        });
        
        self.current_size += size;
    }
}
```

### 7.2 Análisis Asincrónico

Para no bloquear al editor durante análisis pesados:

```rust
use tokio::task;

impl HaloLspServer {
    async fn did_change_async(&mut self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.content_changes[0].text.clone();
        
        // Spawear análisis en thread separado
        let analysis = task::spawn_blocking(move || {
            analyze_full(&text)
        })
        .await
        .unwrap();
        
        // Enviar diagnósticos cuando termina
        self.publish_diagnostics(&uri, analysis.diagnostics);
    }
}
```

---

## 8. Testing

### 8.1 Tests Unitarios

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_completion_at_top_level() {
        let server = HaloLspServer::new();
        let params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("file:///test.halo").unwrap(),
                },
                position: Position { line: 0, character: 0 },
            },
            context: None,
            partial_result_params: Default::default(),
            work_done_progress_params: Default::default(),
        };
        
        let items = completion(&server, params);
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "print"));
    }
    
    #[test]
    fn test_definition_finds_function() {
        let server = HaloLspServer::new();
        
        // Abrir documento con función
        server.did_open(DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: Url::parse("file:///test.halo").unwrap(),
                language_id: "halo".to_string(),
                version: 0,
                text: "foo() { return 42 }\nfoo()".to_string(),
            },
        });
        
        // Pedir definición de segundo "foo"
        let loc = definition(&server, GotoDefinitionParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("file:///test.halo").unwrap(),
                },
                position: Position { line: 1, character: 0 },
            },
            partial_result_params: Default::default(),
            work_done_progress_params: Default::default(),
        });
        
        assert!(loc.is_some());
        assert_eq!(loc.unwrap().range.start.line, 0);
    }
}
```

---

## 9. Integración Continua

### 9.1 GitHub Actions para LSP

```yaml
name: LSP Tests
on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build LSP
        run: cargo build --bin halo-lsp --release
      
      - name: Test LSP
        run: cargo test --bin halo-lsp
      
      - name: Lint
        run: cargo clippy --bin halo-lsp
```

---

## 10. Roadmap

### Fase 1 (v0.3.0) - MVP
- [x] Initialize/shutdown
- [x] Document sync (open/change/close)
- [x] Publish diagnostics
- [ ] Completion básico
- [ ] Definition

### Fase 2 (v0.4.0)
- [ ] References
- [ ] Hover
- [ ] Rename
- [ ] Document symbols

### Fase 3 (v0.5.0)
- [ ] Format document
- [ ] Workspace symbols
- [ ] Call hierarchy
- [ ] Code lens

### Fase 4 (v0.6.0+)
- [ ] Snippets
- [ ] Inlay hints
- [ ] Semantic tokens
- [ ] Code actions (quick fixes)
