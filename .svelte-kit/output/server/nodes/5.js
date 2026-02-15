import * as server from '../entries/pages/sverdle/_page.server.ts.js';

export const index = 5;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/sverdle/_page.svelte.js')).default;
export { server };
export const server_id = "src/routes/sverdle/+page.server.ts";
export const imports = ["_app/immutable/nodes/5.BpDLS8MS.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/bNQyYDU7.js","_app/immutable/chunks/BZ3xhaWD.js","_app/immutable/chunks/a6MJfn4m.js","_app/immutable/chunks/CS6rVQ-q.js","_app/immutable/chunks/BnlD1kGp.js","_app/immutable/chunks/DrK46S25.js","_app/immutable/chunks/BRzKQYUk.js","_app/immutable/chunks/Dbw8gDSx.js","_app/immutable/chunks/BLF5Old3.js","_app/immutable/chunks/D9RL6rbe.js","_app/immutable/chunks/CMdNu_qX.js","_app/immutable/chunks/y1z0ACOv.js"];
export const stylesheets = ["_app/immutable/assets/5.C7uEq_1w.css"];
export const fonts = [];
