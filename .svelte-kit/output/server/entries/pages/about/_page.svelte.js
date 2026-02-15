import { h as head } from "../../../chunks/index2.js";
import { r as resolve } from "../../../chunks/server2.js";
import { a as attr } from "../../../chunks/attributes.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    head("cwls5q", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>About</title>`);
      });
      $$renderer3.push(`<meta name="description" content="About this app"/>`);
    });
    $$renderer2.push(`<div class="text-column"><h1>About this app</h1> <p>This is a <a href="https://svelte.dev/docs/kit">SvelteKit</a> app. You can make your own by typing
		the following into your command line and following the prompts:</p> <pre>npx sv create</pre> <p>The page you're looking at is purely static HTML, with no client-side interactivity needed.
		Because of that, we don't need to load any JavaScript. Try viewing the page's source, or opening
		the devtools network panel and reloading.</p> <p>The <a${attr("href", resolve("/sverdle"))}>Sverdle</a> page illustrates SvelteKit's data loading and form handling.
		Try using it with JavaScript disabled!</p></div>`);
  });
}
export {
  _page as default
};
