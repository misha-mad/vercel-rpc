import { h as head, z as attr_class, F as ensure_array_like, G as bind_props, s as stringify } from "../../../chunks/index2.js";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils.js";
import { a as attr, c as clsx } from "../../../chunks/attributes.js";
import { r as resolve } from "../../../chunks/server2.js";
import "../../../chunks/state.svelte.js";
import { e as escape_html } from "../../../chunks/escaping.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { data, form = void 0 } = $$props;
    let won = data.answers.at(-1) === "xxxxx";
    let i = won ? -1 : data.answers.length;
    let currentGuess = data.guesses[i] || "";
    let submittable = currentGuess.length === 5;
    const { classnames, description } = (() => {
      let classnames2 = {};
      let description2 = {};
      data.answers.forEach((answer, i2) => {
        const guess = data.guesses[i2];
        for (let i3 = 0; i3 < 5; i3 += 1) {
          const letter = guess[i3];
          if (answer[i3] === "x") {
            classnames2[letter] = "exact";
            description2[letter] = "correct";
          } else if (!classnames2[letter]) {
            classnames2[letter] = answer[i3] === "c" ? "close" : "missing";
            description2[letter] = answer[i3] === "c" ? "present" : "absent";
          }
        }
      });
      return { classnames: classnames2, description: description2 };
    })();
    head("115oxdy", $$renderer2, ($$renderer3) => {
      $$renderer3.title(($$renderer4) => {
        $$renderer4.push(`<title>Sverdle</title>`);
      });
      $$renderer3.push(`<meta name="description" content="A Wordle clone written in SvelteKit" class="svelte-115oxdy"/>`);
    });
    $$renderer2.push(`<h1 class="visually-hidden svelte-115oxdy">Sverdle</h1> <form method="post" action="?/enter" class="svelte-115oxdy"><a class="how-to-play svelte-115oxdy"${attr("href", resolve("/sverdle/how-to-play"))}>How to play</a> <div${attr_class("grid svelte-115oxdy", void 0, { "playing": !won, "bad-guess": form?.badGuess })}><!--[-->`);
    const each_array = ensure_array_like(Array.from(Array(6).keys()));
    for (let $$index_1 = 0, $$length = each_array.length; $$index_1 < $$length; $$index_1++) {
      let row = each_array[$$index_1];
      const current = row === i;
      $$renderer2.push(`<h2 class="visually-hidden svelte-115oxdy">Row ${escape_html(row + 1)}</h2> <div${attr_class("row svelte-115oxdy", void 0, { "current": current })}><!--[-->`);
      const each_array_1 = ensure_array_like(Array.from(Array(5).keys()));
      for (let $$index = 0, $$length2 = each_array_1.length; $$index < $$length2; $$index++) {
        let column = each_array_1[$$index];
        const guess = current ? currentGuess : data.guesses[row];
        const answer = data.answers[row]?.[column];
        const value = guess?.[column] ?? "";
        const selected = current && column === guess.length;
        const exact = answer === "x";
        const close = answer === "c";
        const missing = answer === "_";
        $$renderer2.push(`<div${attr_class("letter svelte-115oxdy", void 0, {
          "exact": exact,
          "close": close,
          "missing": missing,
          "selected": selected
        })}>${escape_html(value)} <span class="visually-hidden svelte-115oxdy">`);
        if (exact) {
          $$renderer2.push("<!--[-->");
          $$renderer2.push(`(correct)`);
        } else if (close) {
          $$renderer2.push("<!--[1-->");
          $$renderer2.push(`(present)`);
        } else if (missing) {
          $$renderer2.push("<!--[2-->");
          $$renderer2.push(`(absent)`);
        } else {
          $$renderer2.push("<!--[!-->");
          $$renderer2.push(`empty`);
        }
        $$renderer2.push(`<!--]--></span> <input name="guess"${attr("disabled", !current, true)} type="hidden"${attr("value", value)} class="svelte-115oxdy"/></div>`);
      }
      $$renderer2.push(`<!--]--></div>`);
    }
    $$renderer2.push(`<!--]--></div> <div class="controls svelte-115oxdy">`);
    if (won || data.answers.length >= 6) {
      $$renderer2.push("<!--[-->");
      if (!won && data.answer) {
        $$renderer2.push("<!--[-->");
        $$renderer2.push(`<p class="svelte-115oxdy">the answer was "${escape_html(data.answer)}"</p>`);
      } else {
        $$renderer2.push("<!--[!-->");
      }
      $$renderer2.push(`<!--]--> <button data-key="enter" class="restart selected svelte-115oxdy" formaction="?/restart">${escape_html(won ? "you won :)" : `game over :(`)} play again?</button>`);
    } else {
      $$renderer2.push("<!--[!-->");
      $$renderer2.push(`<div class="keyboard svelte-115oxdy"><button data-key="enter"${attr("disabled", !submittable, true)}${attr_class("svelte-115oxdy", void 0, { "selected": submittable })}>enter</button> <button data-key="backspace" formaction="?/update" name="key" value="backspace" class="svelte-115oxdy">back</button> <!--[-->`);
      const each_array_2 = ensure_array_like(["qwertyuiop", "asdfghjkl", "zxcvbnm"]);
      for (let $$index_3 = 0, $$length = each_array_2.length; $$index_3 < $$length; $$index_3++) {
        let row = each_array_2[$$index_3];
        $$renderer2.push(`<div class="row svelte-115oxdy"><!--[-->`);
        const each_array_3 = ensure_array_like(row);
        for (let index = 0, $$length2 = each_array_3.length; index < $$length2; index++) {
          let letter = each_array_3[index];
          $$renderer2.push(`<button${attr("data-key", letter)}${attr_class(clsx(classnames[letter]), "svelte-115oxdy")}${attr("disabled", submittable, true)} formaction="?/update" name="key"${attr("value", letter)}${attr("aria-label", `${stringify(letter)} ${stringify(description[letter] || "")}`)}>${escape_html(letter)}</button>`);
        }
        $$renderer2.push(`<!--]--></div>`);
      }
      $$renderer2.push(`<!--]--></div>`);
    }
    $$renderer2.push(`<!--]--></div></form> `);
    if (won) {
      $$renderer2.push("<!--[-->");
      $$renderer2.push(`<div style="position: absolute; left: 50%; top: 30%" class="svelte-115oxdy"></div>`);
    } else {
      $$renderer2.push("<!--[!-->");
    }
    $$renderer2.push(`<!--]-->`);
    bind_props($$props, { form });
  });
}
export {
  _page as default
};
