// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.
// import("./index.js")
//   .catch(e => console.error("Error importing `index.js`:", e));

import("./index.js")
  .then((module) => {
    // Attach imported function to global scope if needed directly
    window.drawBoard = module.drawBoard;
  })
  .catch(e => console.error("Error importing `index.js`:", e));
