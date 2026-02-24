import { createHighlighterCore } from 'shiki/core';
import { createJavaScriptRegexEngine } from 'shiki/engine/javascript';
import type { HighlighterCore } from 'shiki/core';

let highlighter: HighlighterCore | undefined;

async function getHighlighter(): Promise<HighlighterCore> {
	if (!highlighter) {
		highlighter = await createHighlighterCore({
			themes: [import('@shikijs/themes/ayu-dark'), import('@shikijs/themes/houston')],
			langs: [import('@shikijs/langs/rust'), import('@shikijs/langs/typescript')],
			engine: createJavaScriptRegexEngine()
		});
	}
	return highlighter;
}

export async function highlightCode(code: string, lang: 'rust' | 'typescript'): Promise<string> {
	const hl = await getHighlighter();
	const theme = lang === 'rust' ? 'ayu-dark' : 'houston';
	return hl.codeToHtml(code, { lang, theme });
}
