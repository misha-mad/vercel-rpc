// Local dev mock for the Rust /api/hello lambda.
// In production, Vercel routes /api/hello to the Rust binary.
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async ({ url }) => {
  const raw = url.searchParams.get("input");
  const name = raw ? JSON.parse(raw) : "World";
  return json({
    result: { type: "response", data: `Hello, ${name} from Rust on Vercel!` },
  });
};
