import * as universal from '../entries/pages/_page.ts.js';

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/_page.svelte.js')).default;
export { universal };
export const universal_id = "src/routes/+page.ts";
export const imports = ["_app/immutable/nodes/2.DfwJUw_9.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/CJxNQnha.js","_app/immutable/chunks/bNQyYDU7.js","_app/immutable/chunks/BnlD1kGp.js","_app/immutable/chunks/DrK46S25.js","_app/immutable/chunks/BZ3xhaWD.js","_app/immutable/chunks/BRzKQYUk.js"];
export const stylesheets = ["_app/immutable/assets/2.C7W8Uifw.css"];
export const fonts = [];
