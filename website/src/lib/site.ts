const base = import.meta.env.BASE_URL.replace(/\/$/, '');

/** Prefix a root-relative path with the deployed base path. */
export function withBase(path: string): string {
	if (/^(?:[a-z]+:)?\/\//i.test(path) || path.startsWith('#') || path.startsWith('mailto:')) {
		return path;
	}
	return `${base}/${path.replace(/^\//, '')}`;
}

export const GITHUB_URL = 'https://github.com/Angelito91/halo';
export const GITHUB_BLOB = `${GITHUB_URL}/blob/main`;
export const GITHUB_EDIT = `${GITHUB_URL}/edit/main/website`;
