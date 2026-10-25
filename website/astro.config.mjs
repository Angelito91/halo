// @ts-check
import { readFileSync } from 'node:fs';
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

const haloGrammar = JSON.parse(
	readFileSync(new URL('./grammars/halo.tmLanguage.json', import.meta.url), 'utf8'),
);

// https://astro.build/config
export default defineConfig({
	site: 'https://angelito91.github.io',
	base: '/halo',
	trailingSlash: 'always',
	integrations: [
		starlight({
			title: 'Halo',
			description:
				'Un lenguaje interpretado, dinámico y minimalista para aprender cómo funciona un lenguaje y escribir scripts simples.',
			logo: {
				src: './src/assets/halo.png',
				alt: 'Halo',
			},
			favicon: '/favicon.png',
			locales: {
				root: { label: 'Español', lang: 'es' },
			},
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/Angelito91/halo',
				},
			],
			editLink: {
				baseUrl: 'https://github.com/Angelito91/halo/edit/main/website/',
			},
			lastUpdated: true,
			customCss: ['./src/styles/global.css'],
			tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
			expressiveCode: {
				shiki: { langs: [haloGrammar] },
				styleOverrides: {
					borderRadius: '0.75rem',
				},
			},
			components: {
				Hero: './src/components/Hero.astro',
			},
			sidebar: [
				{
					label: 'Empezando',
					items: [
						{ slug: 'empezando/que-es-halo' },
						{ slug: 'empezando/instalacion' },
						{ slug: 'empezando/primer-programa' },
						{ slug: 'empezando/cli' },
					],
				},
				{
					label: 'Guía del lenguaje',
					items: [
						{ slug: 'guia/estructura' },
						{ slug: 'guia/variables-y-tipos' },
						{ slug: 'guia/operadores' },
						{ slug: 'guia/condicionales' },
						{ slug: 'guia/bucles' },
						{ slug: 'guia/funciones' },
						{ slug: 'guia/scope' },
						{ slug: 'guia/arrays' },
						{ slug: 'guia/strings' },
						{ slug: 'guia/estilo' },
					],
				},
				{
					label: 'Referencia',
					items: [
						{ slug: 'referencia/builtins' },
						{ slug: 'referencia/errores' },
						{ slug: 'referencia/precedencia' },
					],
				},
				{
					label: 'Ejemplos',
					items: [
						{ slug: 'ejemplos' },
						{ slug: 'ejemplos/proyectos' },
					],
				},
				{
					label: 'Proyecto',
					items: [
						{ slug: 'proyecto/arquitectura' },
						{ slug: 'proyecto/testing' },
						{ slug: 'proyecto/roadmap' },
						{ slug: 'proyecto/contribuir' },
					],
				},
			],
			head: [
				{
					tag: 'meta',
					attrs: {
						property: 'og:image',
						content: 'https://angelito91.github.io/halo/og.png',
					},
				},
				{
					tag: 'meta',
					attrs: { property: 'og:image:width', content: '1200' },
				},
				{
					tag: 'meta',
					attrs: { property: 'og:image:height', content: '630' },
				},
				{
					tag: 'meta',
					attrs: {
						name: 'twitter:image',
						content: 'https://angelito91.github.io/halo/og.png',
					},
				},
				{
					tag: 'meta',
					attrs: { name: 'theme-color', content: '#07111c' },
				},
			],
		}),
	],
});
