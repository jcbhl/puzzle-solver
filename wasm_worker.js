import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

function callFFI(ffiImpl) {
  const ffiResult = ffiImpl();
  return Comlink.transfer(ffiResult);
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('./pkg/puzzle_solver.js');
      await singleThread.default();
      const ffiResult = singleThread();
      return Comlink.transfer(ffiResult);
    })(),
    (async () => {
      // If threads are unsupported in this browser, skip this handler.
      if (!(await threads())) return;
      const multiThread = await import(
        './pkg-parallel/puzzle_solver.js'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      const ffiResult = multiThread();
      return Comlink.transfer(ffiResult);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});
