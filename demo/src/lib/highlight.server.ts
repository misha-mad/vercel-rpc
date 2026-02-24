import { createHighlighterCore } from 'shiki/core';
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';
import type { HighlighterCore } from 'shiki/core';

let highlighter: HighlighterCore | undefined;

async function getHighlighter(): Promise<HighlighterCore> {
	if (!highlighter) {
		highlighter = await createHighlighterCore({
			themes: [import('@shikijs/themes/github-dark-high-contrast')],
			langs: [import('@shikijs/langs/rust'), import('@shikijs/langs/typescript'), import('@shikijs/langs/shellscript')],
			engine: createJavaScriptRegexEngine()
		});
	}
	return highlighter;
}

const langLabels: Record<string, string> = {
	rust: 'rust',
	typescript: 'ts',
	shellscript: 'sh'
};

export async function highlightCode(code: string, lang: 'rust' | 'typescript' | 'shellscript'): Promise<string> {
	const hl = await getHighlighter();
	const html = hl.codeToHtml(code, { lang, theme: 'github-dark-high-contrast' });
	const label = langLabels[lang] ?? lang;
	return html.replace('<pre', `<pre data-lang="${label}"`);
}
