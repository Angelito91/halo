import { existsSync, readFileSync } from 'node:fs';
import { resolve } from 'node:path';

const candidates = [
	// `astro build` / `astro dev` are normally run from `website/`.
	resolve(process.cwd(), '..', 'Cargo.toml'),
	// Running from the repository root with `--root website`.
	resolve(process.cwd(), 'Cargo.toml'),
	// Fallback for unusual setups where the cwd is elsewhere.
	resolve(process.cwd(), 'website', '..', 'Cargo.toml'),
];

const cargoPath = candidates.find((path) => existsSync(path));

if (!cargoPath) {
	throw new Error(
		`Could not find the project's Cargo.toml. Looked in:\n${candidates.join('\n')}`,
	);
}

const cargoToml = readFileSync(cargoPath, 'utf8');
const match = /^version\s*=\s*"([^"]+)"/m.exec(cargoToml);

/** Version declared by the project's Cargo.toml, read at build time. */
export const VERSION = match?.[1] ?? '0.0.0';
