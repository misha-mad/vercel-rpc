// Local dev mock for the Rust /api/time lambda.
// In production, Vercel routes /api/time to the Rust binary.
import { json } from "@sveltejs/kit";
import type { RequestHandler } from "./$types";

export const GET: RequestHandler = async () => {
  return json({
    result: {
      type: "response",
      data: {
        timestamp: Math.floor(Date.now() / 1000),
        message: "Current server time in seconds since epoch",
      },
    },
  });
};
