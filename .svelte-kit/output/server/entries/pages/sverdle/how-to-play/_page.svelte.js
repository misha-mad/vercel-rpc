import { h as head } from "../../../../chunks/index2.js";
function _page($$renderer) {
  head("awtuxs", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>How to play Sverdle</title>`);
    });
    $$renderer2.push(`<meta name="description" content="How to play Sverdle"/>`);
  });
  $$renderer.push(`<div class="text-column"><h1>How to play Sverdle</h1> <p>Sverdle is a clone of <a href="https://www.nytimes.com/games/wordle/index.html">Wordle</a>, the
		word guessing game. To play, enter a five-letter English word. For example:</p> <div class="example svelte-awtuxs"><span class="close svelte-awtuxs">r</span> <span class="missing svelte-awtuxs">i</span> <span class="close svelte-awtuxs">t</span> <span class="missing svelte-awtuxs">z</span> <span class="exact svelte-awtuxs">y</span></div> <p class="svelte-awtuxs">The <span class="exact svelte-awtuxs">y</span> is in the right place. <span class="close svelte-awtuxs">r</span> and <span class="close svelte-awtuxs">t</span> are the right letters, but in the wrong place. The other letters are wrong, and can be discarded.
		Let's make another guess:</p> <div class="example svelte-awtuxs"><span class="exact svelte-awtuxs">p</span> <span class="exact svelte-awtuxs">a</span> <span class="exact svelte-awtuxs">r</span> <span class="exact svelte-awtuxs">t</span> <span class="exact svelte-awtuxs">y</span></div> <p>This time we guessed right! You have <strong>six</strong> guesses to get the word.</p> <p>Unlike the original Wordle, Sverdle runs on the server instead of in the browser, making it
		impossible to cheat. It uses <code>&lt;form></code> and cookies to submit data, meaning you can
		even play with JavaScript disabled!</p></div>`);
}
export {
  _page as default
};
