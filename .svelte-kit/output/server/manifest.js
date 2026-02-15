export const manifest = (() => {
function __memo(fn) {
	let value;
	return () => value ??= (value = fn());
}

return {
	appDir: "_app",
	appPath: "_app",
	assets: new Set(["favicon.svg","robots.txt"]),
	mimeTypes: {".svg":"image/svg+xml",".txt":"text/plain"},
	_: {
		client: {start:"_app/immutable/entry/start.BoV9tQm5.js",app:"_app/immutable/entry/app.g6dvfdC9.js",imports:["_app/immutable/entry/start.BoV9tQm5.js","_app/immutable/chunks/BLF5Old3.js","_app/immutable/chunks/D9RL6rbe.js","_app/immutable/chunks/bNQyYDU7.js","_app/immutable/chunks/BZ3xhaWD.js","_app/immutable/chunks/CMdNu_qX.js","_app/immutable/entry/app.g6dvfdC9.js","_app/immutable/chunks/bNQyYDU7.js","_app/immutable/chunks/BZ3xhaWD.js","_app/immutable/chunks/DsnmJJEf.js","_app/immutable/chunks/D9RL6rbe.js","_app/immutable/chunks/a6MJfn4m.js","_app/immutable/chunks/CS6rVQ-q.js","_app/immutable/chunks/Dbw8gDSx.js"],stylesheets:[],fonts:[],uses_env_dynamic_public:false},
		nodes: [
			__memo(() => import('./nodes/0.js')),
			__memo(() => import('./nodes/1.js')),
			__memo(() => import('./nodes/4.js')),
			__memo(() => import('./nodes/5.js'))
		],
		remotes: {
			
		},
		routes: [
			{
				id: "/rpc",
				pattern: /^\/rpc\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 2 },
				endpoint: null
			},
			{
				id: "/sverdle",
				pattern: /^\/sverdle\/?$/,
				params: [],
				page: { layouts: [0,], errors: [1,], leaf: 3 },
				endpoint: null
			}
		],
		prerendered_routes: new Set(["/","/about","/sverdle/how-to-play"]),
		matchers: async () => {
			
			return {  };
		},
		server_assets: {}
	}
}
})();
