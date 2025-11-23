import {
    setModuleArgs
} from '@undecaf/zbar-wasm';

// setModuleArgs({
//     locateFile: (file) => "/assets/zbar-dxhbfb38b727d251423.wasm"
// });

function loadBarcodeDetectorPolyfill(args) {
    setModuleArgs(args);
    import('@preflower/barcode-detector-polyfill').then(({
        BarcodeDetectorPolyfill
    }) => {
        window.BarcodeDetector = window.BarcodeDetector || BarcodeDetectorPolyfill;
    });
}

import {
    BarcodeDetectorPolyfill
} from '@preflower/barcode-detector-polyfill';

try {
    window['loadBarcodeDetectorPolyfill'] = (args) => {};
    window['BarcodeDetector'].getSupportedFormats()
} catch {
    window['loadBarcodeDetectorPolyfill'] = loadBarcodeDetectorPolyfill;
}