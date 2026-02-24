import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	streamRust: {
		lang: 'rust',
		code: `#[rpc_stream]
async fn chat(input: ChatInput, tx: StreamSender) {
    for token in generate_tokens(&input.prompt) {
        tx.send(token).await;
    }
}`
	},
	streamTs: {
		lang: 'typescript',
		code: `const stream = rpc.stream('chat', {
  prompt: 'Explain Rust ownership'
});

for await (const token of stream) {
  console.log(token); // each chunk
}`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(
		entries.map(([, { code, lang }]) => highlightCode(code, lang))
	);
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
