import init, { showSections } from './pkg/wasm_custom_sections.js';

window.addEventListener('load', async () => {
    await init();
    showSections();
});
