import * as Comlink from 'comlink';

(async function init() {
  // Create a separate thread from wasm-worker.js and get a proxy to its handlers.
  let handlers = await Comlink.wrap(
    new Worker(new URL('./wasm_worker.js', import.meta.url), {
      type: 'module'
    })
  ).handlers;

  function setupBtn(id) {
    // Handlers are named in the same way as buttons.
    let handler = handlers[id];
    // If handler doesn't exist, it's not supported.
    if (!handler) return;
    // Assign onclick handler + enable the button.
    Object.assign(document.getElementById(id), {
      async onclick() {
        let rawImageData = await handler(maxIterations);
      },
      disabled: false
    });
  }

  setupBtn('singleThread');
  if (await handlers.supportsThreads) {
    setupBtn('multiThread');
  }
})();