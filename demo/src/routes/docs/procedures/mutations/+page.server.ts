import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	echoRust: {
		lang: 'rust',
		code: `#[derive(Deserialize)]
pub struct EchoInput {
    pub message: String,
    pub uppercase: bool,
}

#[derive(Serialize)]
pub struct EchoOutput {
    pub message: String,
    pub length: usize,
}

#[rpc_mutation]
async fn echo(input: EchoInput) -> EchoOutput {
    let msg = if input.uppercase {
        input.message.to_uppercase()
    } else {
        input.message
    };
    EchoOutput { message: msg.clone(), length: msg.len() }
}`
	},
	echoTs: {
		lang: 'typescript',
		code: `const echo = createMutation(rpc, "echo");

// Trigger explicitly:
echo.mutate({ message: "Hello", uppercase: true });

echo.data       // EchoOutput | undefined
echo.isLoading  // boolean (loading)
echo.isError    // boolean
echo.error      // RpcError | undefined`
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
