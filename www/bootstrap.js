// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.

import("./connect4.js")
  .then((module) => {
    window.drawBoard = module.drawBoard;
    window.onLoad = module.onLoad;
    window.newGame = module.newGame;
  })
  .catch(e => console.error("Error importing `connect4.js`:", e));


import("./toot.js")
  .then((module) => {
    window.drawBoardToot = module.drawBoardToot;
  })
  .catch(e => console.error("Error importing `toot.js`:", e));

