# Documentación de Halo

Sitio oficial de documentación del lenguaje Halo, construido con
[Astro](https://astro.build/) y [Starlight](https://starlight.astro.build/).

- **Producción:** <https://angelito91.github.io/halo/>
- **Contenido:** español
- **Base path:** `/halo`
- **Fuente de datos:** `../examples/` y `../Cargo.toml` (se leen en build time)

## Desarrollo local

```bash
cd website
bun install
bun run dev
```

El servidor arranca en <http://localhost:4321/halo/>.

## Comandos

| Comando | Acción |
|---|---|
| `bun install` | Instala dependencias |
| `bun run dev` | Servidor de desarrollo |
| `bun run build` | Build de producción en `dist/` |
| `bun run preview` | Previsualiza el build |
| `bun run check` | Comprueba tipos y contenido |

> `astro check` requiere TypeScript 6. El proyecto fija `typescript@^6` por esa
> razón; TypeScript 7 todavía no está soportado por `@astrojs/check`.

## Estructura

```text
website/
├── astro.config.mjs        # Starlight, sidebar, base, gramática
├── grammars/
│   └── halo.tmLanguage.json
├── public/
│   ├── favicon.png
│   └── og.png              # Imagen Open Graph 1200×630
└── src/
    ├── assets/halo.png     # Logo/mascota
    ├── components/
    │   ├── Hero.astro      # Hero de la portada
    │   └── Example.astro   # Incluye archivos reales de ../examples/
    ├── content/
    │   ├── docs/           # Páginas en Markdown/MDX
    │   └── i18n/es.json    # Textos de la interfaz
    ├── lib/
    │   ├── site.ts         # withBase() y URLs de GitHub
    │   └── version.ts      # Lee la versión de ../Cargo.toml
    └── styles/global.css   # Tema y tipografía
```

## Añadir una página

1. Crea el archivo en `src/content/docs/<seccion>/<slug>.md` o `.mdx`.
2. Añade el frontmatter:

   ```yaml
   ---
   title: Título de la página
   description: Una frase para SEO y resultados de búsqueda.
   ---
   ```

3. Añade `{ slug: 'seccion/slug' }` al sidebar en `astro.config.mjs`.
4. Usa `halo` en los bloques de código para el resaltado.
5. Comprueba el build con `bun run build`.

### Enlaces internos

El sitio se sirve con `trailingSlash: 'always'` y base `/halo`. Cada página es
una URL con barra final (`/halo/guia/arrays/`), así que los enlaces relativos
deben llevar **un `../` extra** respecto a la ruta del archivo fuente:

```md
<!-- En guia/arrays.md -->
[Strings](../strings/)
[Referencia](../../referencia/builtins/)
```

Las páginas índice (`index.mdx` y `ejemplos/index.mdx`) son la excepción: su
URL coincide con su directorio y los enlaces relativos funcionan tal cual.

### Componentes

- `<Example file="03_factorial.halo" from={69} to={84} />` incluye un fragmento
  real de `../examples/`. No copies el código a mano.
- `Hero.astro` es el hero de la portada; usa `withBase()` para enlaces
  absolutos.
- Los `LinkCard` y enlaces Markdown de la portada usan rutas relativas.

## Versión

El badge de versión y el aviso de estado de la portada leen `../Cargo.toml` en
build time (`src/lib/version.ts`). Si cambias la versión del compilador, esos
elementos se actualizan solos en el siguiente build.

## Despliegue

El workflow `.github/workflows/docs.yml`:

1. Instala Bun y las dependencias con `--frozen-lockfile`.
2. Ejecuta `bun run build` en `website/`.
3. Sube `website/dist` como artefacto de GitHub Pages.
4. Despliega con `actions/deploy-pages`.

En GitHub, configura **Settings → Pages → Source: GitHub Actions**.

Los PR que tocan `website/**` construyen el sitio, pero no despliegan.

## Gramática

`grammars/halo.tmLanguage.json` define el resaltado para el lenguaje `halo`.
Se registra en `astro.config.mjs` dentro de `expressiveCode.shiki.langs`. Si
añades sintaxis nueva, actualiza los patrones de la gramática.

## Licencia

MPL 2.0, igual que el resto del proyecto.
