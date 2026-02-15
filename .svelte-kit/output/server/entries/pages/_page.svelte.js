import { e as attr_style, s as stringify, h as head } from "../../chunks/index2.js";
import { n as noop, b as deferred } from "../../chunks/context.js";
import { M as state, r as render_effect, J as set, a as get } from "../../chunks/runtime.js";
import "clsx";
import { e as escape_html } from "../../chunks/escaping.js";
import { a as attr } from "../../chunks/attributes.js";
const now = () => Date.now();
const raf = {
  // don't access requestAnimationFrame eagerly outside method
  // this allows basic testing of user code without JSDOM
  // bunder will eval and remove ternary when the user's app is built
  tick: (
    /** @param {any} _ */
    (_) => noop()
  ),
  now: () => now(),
  tasks: /* @__PURE__ */ new Set()
};
function loop(callback) {
  let task;
  if (raf.tasks.size === 0) ;
  return {
    promise: new Promise((fulfill) => {
      raf.tasks.add(task = { c: callback, f: fulfill });
    }),
    abort() {
      raf.tasks.delete(task);
    }
  };
}
const welcomeFallback = "/_app/immutable/assets/svelte-welcome.VNiyy3gC.png";
const welcome = "/_app/immutable/assets/svelte-welcome.0pIiHnVF.webp";
function is_date(obj) {
  return Object.prototype.toString.call(obj) === "[object Date]";
}
function tick_spring(ctx, last_value, current_value, target_value) {
  if (typeof current_value === "number" || is_date(current_value)) {
    const delta = target_value - current_value;
    const velocity = (current_value - last_value) / (ctx.dt || 1 / 60);
    const spring = ctx.opts.stiffness * delta;
    const damper = ctx.opts.damping * velocity;
    const acceleration = (spring - damper) * ctx.inv_mass;
    const d = (velocity + acceleration) * ctx.dt;
    if (Math.abs(d) < ctx.opts.precision && Math.abs(delta) < ctx.opts.precision) {
      return target_value;
    } else {
      ctx.settled = false;
      return is_date(current_value) ? new Date(current_value.getTime() + d) : current_value + d;
    }
  } else if (Array.isArray(current_value)) {
    return current_value.map(
      (_, i) => (
        // @ts-ignore
        tick_spring(ctx, last_value[i], current_value[i], target_value[i])
      )
    );
  } else if (typeof current_value === "object") {
    const next_value = {};
    for (const k in current_value) {
      next_value[k] = tick_spring(ctx, last_value[k], current_value[k], target_value[k]);
    }
    return next_value;
  } else {
    throw new Error(`Cannot spring ${typeof current_value} values`);
  }
}
class Spring {
  #stiffness = state(0.15);
  #damping = state(0.8);
  #precision = state(0.01);
  #current;
  #target;
  #last_value = (
    /** @type {T} */
    void 0
  );
  #last_time = 0;
  #inverse_mass = 1;
  #momentum = 0;
  /** @type {import('../internal/client/types').Task | null} */
  #task = null;
  /** @type {ReturnType<typeof deferred> | null} */
  #deferred = null;
  /**
   * @param {T} value
   * @param {SpringOpts} [options]
   */
  constructor(value, options = {}) {
    this.#current = state(value);
    this.#target = state(value);
    if (typeof options.stiffness === "number") this.#stiffness.v = clamp(options.stiffness, 0, 1);
    if (typeof options.damping === "number") this.#damping.v = clamp(options.damping, 0, 1);
    if (typeof options.precision === "number") this.#precision.v = options.precision;
  }
  /**
   * Create a spring whose value is bound to the return value of `fn`. This must be called
   * inside an effect root (for example, during component initialisation).
   *
   * ```svelte
   * <script>
   * 	import { Spring } from 'svelte/motion';
   *
   * 	let { number } = $props();
   *
   * 	const spring = Spring.of(() => number);
   * <\/script>
   * ```
   * @template U
   * @param {() => U} fn
   * @param {SpringOpts} [options]
   */
  static of(fn, options) {
    const spring = new Spring(fn(), options);
    render_effect(() => {
      spring.set(fn());
    });
    return spring;
  }
  /** @param {T} value */
  #update(value) {
    set(this.#target, value);
    this.#current.v ??= value;
    this.#last_value ??= this.#current.v;
    if (!this.#task) {
      this.#last_time = raf.now();
      var inv_mass_recovery_rate = 1e3 / (this.#momentum * 60);
      this.#task ??= loop((now2) => {
        this.#inverse_mass = Math.min(this.#inverse_mass + inv_mass_recovery_rate, 1);
        const elapsed = Math.min(now2 - this.#last_time, 1e3 / 30);
        const ctx = {
          inv_mass: this.#inverse_mass,
          opts: {
            stiffness: this.#stiffness.v,
            damping: this.#damping.v,
            precision: this.#precision.v
          },
          settled: true,
          dt: elapsed * 60 / 1e3
        };
        var next = tick_spring(ctx, this.#last_value, this.#current.v, this.#target.v);
        this.#last_value = this.#current.v;
        this.#last_time = now2;
        set(this.#current, next);
        if (ctx.settled) {
          this.#task = null;
        }
        return !ctx.settled;
      });
    }
    return this.#task.promise;
  }
  /**
   * Sets `spring.target` to `value` and returns a `Promise` that resolves if and when `spring.current` catches up to it.
   *
   * If `options.instant` is `true`, `spring.current` immediately matches `spring.target`.
   *
   * If `options.preserveMomentum` is provided, the spring will continue on its current trajectory for
   * the specified number of milliseconds. This is useful for things like 'fling' gestures.
   *
   * @param {T} value
   * @param {SpringUpdateOpts} [options]
   */
  set(value, options) {
    this.#deferred?.reject(new Error("Aborted"));
    if (options?.instant || this.#current.v === void 0) {
      this.#task?.abort();
      this.#task = null;
      set(this.#current, set(this.#target, value));
      this.#last_value = value;
      return Promise.resolve();
    }
    if (options?.preserveMomentum) {
      this.#inverse_mass = 0;
      this.#momentum = options.preserveMomentum;
    }
    var d = this.#deferred = deferred();
    d.promise.catch(noop);
    this.#update(value).then(() => {
      if (d !== this.#deferred) return;
      d.resolve(void 0);
    });
    return d.promise;
  }
  get current() {
    return get(this.#current);
  }
  get damping() {
    return get(this.#damping);
  }
  set damping(v) {
    set(this.#damping, clamp(v, 0, 1));
  }
  get precision() {
    return get(this.#precision);
  }
  set precision(v) {
    set(this.#precision, v);
  }
  get stiffness() {
    return get(this.#stiffness);
  }
  set stiffness(v) {
    set(this.#stiffness, clamp(v, 0, 1));
  }
  get target() {
    return get(this.#target);
  }
  set target(v) {
    this.set(v);
  }
}
function clamp(n, min, max) {
  return Math.max(min, Math.min(max, n));
}
function Counter($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const count = new Spring(0);
    const offset = modulo(count.current, 1);
    function modulo(n, m) {
      return (n % m + m) % m;
    }
    $$renderer2.push(`<div class="counter svelte-1czwagy"><button aria-label="Decrease the counter by one" class="svelte-1czwagy"><svg aria-hidden="true" viewBox="0 0 1 1" class="svelte-1czwagy"><path d="M0,0.5 L1,0.5" class="svelte-1czwagy"></path></svg></button> <div class="counter-viewport svelte-1czwagy"><div class="counter-digits svelte-1czwagy"${attr_style(`transform: translate(0, ${stringify(100 * offset)}%)`)}><strong class="hidden svelte-1czwagy" aria-hidden="true">${escape_html(Math.floor(count.current + 1))}</strong> <strong class="svelte-1czwagy">${escape_html(Math.floor(count.current))}</strong></div></div> <button aria-label="Increase the counter by one" class="svelte-1czwagy"><svg aria-hidden="true" viewBox="0 0 1 1" class="svelte-1czwagy"><path d="M0,0.5 L1,0.5 M0.5,0 L0.5,1" class="svelte-1czwagy"></path></svg></button></div>`);
  });
}
function _page($$renderer) {
  head("1uha8ag", $$renderer, ($$renderer2) => {
    $$renderer2.title(($$renderer3) => {
      $$renderer3.push(`<title>Home</title>`);
    });
    $$renderer2.push(`<meta name="description" content="Svelte demo app"/>`);
  });
  $$renderer.push(`<section class="svelte-1uha8ag"><h1 class="svelte-1uha8ag"><span class="welcome svelte-1uha8ag"><picture><source${attr("srcset", welcome)} type="image/webp"/> <img${attr("src", welcomeFallback)} alt="Welcome" class="svelte-1uha8ag"/></picture></span> to your new<br/>SvelteKit app</h1> <h2>try editing <strong>src/routes/+page.svelte</strong></h2> `);
  Counter($$renderer);
  $$renderer.push(`<!----></section>`);
}
export {
  _page as default
};
