import { createClient, FetchTransport } from "@rspc/client";
import type { Procedures } from "./rspc";

export const client = createClient<Procedures>({
    transport: new FetchTransport("/api/rspc"),
});
