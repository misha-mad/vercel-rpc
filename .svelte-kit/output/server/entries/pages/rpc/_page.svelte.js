import { a as attr } from "../../../chunks/attributes.js";
import { e as escape_html } from "../../../chunks/escaping.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let name = "SvelteKit";
    let time = "loading...";
    let loading = false;
    $$renderer2.push(`<div class="container svelte-1vknoic"><h1>RPC + SvelteKit + Rust</h1> <p>Server time: <strong>${escape_html(time)}</strong></p> <div class="card svelte-1vknoic"><h2>Type-safe Query</h2> <input type="text"${attr("value", name)} placeholder="Enter your name" class="svelte-1vknoic"/> <button${attr("disabled", loading, true)} class="svelte-1vknoic">${escape_html("Say Hello")}</button> `);
    {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]--></div></div>`);
  });
}
export {
  _page as default
};
