async function test(message) {
    const { greet } = await import('./pkg/wasm_websocket_sample.js');
    greet(message);
}