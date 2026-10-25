<div align="center">
  <img src="halo.png" alt="Halo" width="360">

  # Halo

  **Un lenguaje de scripting interpretado, educativo y minimalista.**

  <sub>Versión 0.2.0 · Licencia MPL 2.0</sub>
</div>

---

## Qué cambió

Halo ya **no compila a binarios nativos**.

- Se eliminó el backend LLVM y la dependencia `inkwell`.
- `halo build` y `halo llvm` ya no existen.
- No hay VM, JIT, `clang`, `llc` ni artefactos `.ll`.
- Hay un solo motor: `lexer → parser → AST → evaluador tree-walking`.

El intérprete conserva todas las funciones del lenguaje: funciones,
recursión, arrays, strings, `for`, `while`, errores con posición y
mensajes de diagnóstico.

Si necesitas consultar el backend anterior, está preservado en la rama
`archive/llvm-backend` y en el tag `llvm-backend-final`.

## Instalación

Requiere Rust stable (2021 edition). No requiere LLVM ni un compilador de C.

```bash
git clone https://github.com/Angelito91/halo.git
cd halo
cargo install --path .
```

Para desarrollo, sin instalar:

```bash
cargo run -- examples/01_hello.halo
```

## Uso

```text
halo <archivo.halo>          # ejecutar (atajo de run)
halo run <archivo.halo>      # ejecutar
halo check <archivo.halo>    # validar sintaxis sin ejecutar
halo tokens <archivo.halo>   # mostrar el flujo de tokens
halo ast <archivo.halo>      # mostrar el AST
halo help                    # ayuda completa
halo version                 # versión
```

`run`, `check`, `tokens` y `ast` aceptan `-v` / `--verbose` para mostrar
el progreso de cada fase.

Ejemplo:

```bash
halo examples/03_factorial.halo
halo check examples/10_arrays.halo
halo tokens examples/01_hello.halo
```

## Sintaxis rápida

### Programa, comentarios y variables

Un programa es una secuencia de sentencias separadas por saltos de línea.
No hay `;` ni función `main`.

```halo
// Comentario de línea
nombre = "Halo"
version = 0.2
activo = true

saludar() {
    print("Hola desde " + nombre)
}

saludar()
```

Las variables se declaran por asignación y pueden cambiar de tipo:

```halo
x = 42        // number (i64)
x = 3.14      // float (f64)
x = "hola"    // string
```

### Tipos

| Tipo | Ejemplos |
|---|---|
| `number` | `0`, `42`, `-10` |
| `float` | `3.14`, `-0.5`, `1.0` |
| `bool` | `true`, `false` |
| `string` | `"hola"`, `'mundo'`, `""` |
| `array` | `[1, 2, 3]`, `[]`, `[1, "dos", true]` |
| `null` | retorno de una función sin `return` |

### Operadores

| Categoría | Operadores |
|---|---|
| Aritmética | `+`, `-`, `*`, `/`, `%` |
| Comparación | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Lógica | `&&`, `\|\|`, `!` (cortocircuito) |
| Unario | `-x`, `!x` |

`+` concatena si hay un string; `*` repite strings. La división entre
enteros es entera (`7 / 2 == 3`); con un float es flotante (`7 / 2.0 == 3.5`).
La aritmética entera está verificada: el overflow y la división por cero
son errores de runtime.

### Condicionales y bucles

```halo
if x < 0 {
    print("negativo")
} else if x == 0 {
    print("cero")
} else {
    print("positivo")
}

i = 0
while i < 5 {
    print(i)
    i = i + 1
}

for n in range(3) {
    print(n)          // 0 1 2
}

for item in [10, 20, 30] {
    if item == 20 { continue }
    print(item)
}
```

`break` y `continue` funcionan en ambos bucles. El `for` acepta arrays y
`range()`, y su variable vive en un scope propio.

### Funciones

```halo
sumar(a, b) {
    return a + b
}

factorial(n) {
    if n <= 1 { return 1 }
    return n * factorial(n - 1)
}

print(sumar(2, 3))        // 5
print(factorial(6))       // 720
```

Las funciones se registran antes de ejecutar el programa, así que puedes
llamarlas antes de su definición textual. Sin `return`, una función
devuelve `null`.

### Arrays

```halo
nums = [1, 2, 3, 4]
nums[0]                     // 1
nums[1:3]                   // [2, 3]
nums[2] = 99                // asignación por índice
len(nums)                   // 4
push(nums, 5)               // [1, 2, 99, 4, 5]
pop(nums)                   // 5
first([10, 20])             // 10
last([10, 20])              // 20
reverse([1, 2, 3])          // [3, 2, 1]
sort([3, 1, 2])             // [1, 2, 3]
join(["a", "b"], "-")       // "a-b"
append([1], [2, 3])         // [1, 2, 3]
contains([1, 2, 3], 2)      // true
index_of([1, 2, 3], 2)      // 1
```

También se acepta la forma con método:

```halo
nums.push(5)
nums.reverse()
nums.contains(3)
```

### Strings

```halo
s = "  Hola Mundo  "
trim(s)                  // "Hola Mundo"
upper(s)                 // "  HOLA MUNDO  "
lower(s)                 // "  hola mundo  "
contains(s, "Mundo")     // true
starts_with(s, "  Ho")   // true
ends_with(s, "  ")       // true
replace(s, "Mundo", "Halo")
substr(s, 2, 4)          // "Hola"
s.trim().upper()         // "HOLA MUNDO"
```

### Built-ins y métodos

Funciones globales: `print`, `println`, `len`, `str`, `int`, `float`,
`abs`, `type`, `range`, `push`, `pop`, `append`, `first`, `last`,
`reverse`, `join`, `contains`, `index_of`, `sort`, `upper`, `lower`,
`trim`, `starts_with`, `ends_with`, `replace`, `substr`.

Métodos de string: `upper`, `lower`, `trim`, `reverse`, `starts_with`,
`ends_with`, `contains`, `index_of`, `replace`, `substr`.

Métodos de array: `push`, `pop`, `first`, `last`, `reverse`, `join`,
`contains`, `index_of`, `sort`.

### Errores

Los errores indican la posición y, cuando aplica, una sugerencia:

```text
error: Undefined variable: 'nombra'
  → script.halo:3:7
  Did you mean 'nombre'?
```

Los límites de seguridad evitan bloquear la máquina: profundidad de
recursión (500) y un millón de iteraciones por programa.

## Arquitectura

```text
src/
├── main.rs            # entrada del binario
├── lib.rs             # API pública
├── cli/               # args, run, check, tokens, ast, help, pipeline
├── diagnostics/       # errores, colores y sugerencias
├── lexer/             # texto → tokens
├── parser/            # tokens → AST
├── interpreter/       # entorno, valores y evaluador tree-walking
└── stdlib/            # builtins y métodos sobre valores ya evaluados
```

El flujo de ejecución es:

```text
archivo → lexer → tokens → parser → AST → evaluador → stdout / error
```

El evaluador solo recorre el AST. Los builtins y métodos viven en
`src/stdlib` y reciben valores ya evaluados, sin conocer el AST ni el
evaluador. El intérprete es síncrono, sin dependencias externas y sin
código nativo.

## Desarrollo

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Tests relevantes:

- `tests/interpreter_tests.rs`, `tests/evaluator_tests.rs`: pipeline completo.
- `tests/arithmetic_tests.rs`, `control_flow_tests.rs`, `function_tests.rs`:
  comportamiento del lenguaje.
- `tests/examples_tests.rs`: ejecuta todos los ejemplos.
- `tests/cli_*_tests.rs`: contrato de la CLI.
- `tests/architecture_tests.rs`: impide que vuelva el backend LLVM.

## Estado y roadmap

Estado actual (v0.2.0): intérprete estable con funciones, recursión,
arrays, strings, slicing, bucles, diagnóstico con posición y CLI
`run`/`check`/`tokens`/`ast`.

Próximos pasos, sin comprometer fechas:

- Ampliar la suite de ejemplos y casos de lenguaje.
- Mejorar mensajes y trazas de error.
- Evaluar un REPL y más built-ins.
- Mantener el proyecto interpreter-only; cualquier backend futuro sería
  un proyecto separado, no una dependencia del intérprete.

## Licencia

MPL 2.0. Consulta [LICENSE](LICENSE).
