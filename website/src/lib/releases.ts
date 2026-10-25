import { GITHUB_URL } from './site';
import { VERSION } from './version';

/** Git tag for the version currently declared in Cargo.toml. */
export const RELEASE_TAG = `v${VERSION}`;

export const RELEASES_URL = `${GITHUB_URL}/releases`;
export const RELEASE_URL = `${RELEASES_URL}/tag/${RELEASE_TAG}`;

export const SOURCE_ZIP_URL = `${GITHUB_URL}/archive/refs/tags/${RELEASE_TAG}.zip`;
export const SOURCE_TARBALL_URL = `${GITHUB_URL}/archive/refs/tags/${RELEASE_TAG}.tar.gz`;

/** Direct download URL for an asset attached to the current release. */
export function releaseAssetUrl(file: string): string {
	return `${RELEASES_URL}/download/${RELEASE_TAG}/${file}`;
}

export const CHECKSUMS_URL = releaseAssetUrl('SHA256SUMS');

export interface ReleaseTarget {
	/** Architecture label shown to the reader. */
	arch: string;
	/** Asset filename published by the release workflow. */
	file: string;
	/** Archive suffix shown next to the download button. */
	kind: 'tar.gz' | 'zip';
}

export interface ReleaseGroup {
	os: string;
	icon: 'linux' | 'apple' | 'seti:windows';
	note: string;
	targets: ReleaseTarget[];
}

/**
 * Keep this list in sync with the build matrix in
 * `.github/workflows/release.yml`.
 */
export const RELEASE_GROUPS: ReleaseGroup[] = [
	{
		os: 'Linux',
		icon: 'linux',
		note: 'Estático (musl), sin dependencias del sistema',
		targets: [
			{
				arch: 'x86_64',
				file: 'halo-x86_64-unknown-linux-musl.tar.gz',
				kind: 'tar.gz',
			},
			{
				arch: 'ARM64',
				file: 'halo-aarch64-unknown-linux-musl.tar.gz',
				kind: 'tar.gz',
			},
		],
	},
	{
		os: 'macOS',
		icon: 'apple',
		note: 'Apple Silicon e Intel',
		targets: [
			{
				arch: 'Apple Silicon',
				file: 'halo-aarch64-apple-darwin.tar.gz',
				kind: 'tar.gz',
			},
			{
				arch: 'Intel',
				file: 'halo-x86_64-apple-darwin.tar.gz',
				kind: 'tar.gz',
			},
		],
	},
	{
		os: 'Windows',
		icon: 'seti:windows',
		note: 'x86_64 y ARM64',
		targets: [
			{
				arch: 'x86_64',
				file: 'halo-x86_64-pc-windows-msvc.zip',
				kind: 'zip',
			},
			{
				arch: 'ARM64',
				file: 'halo-aarch64-pc-windows-msvc.zip',
				kind: 'zip',
			},
		],
	},
];
