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
    window.onLoadToot = module.onLoadToot;
    window.newGameToot = module.newGameToot;
  })
  .catch(e => console.error("Error importing `toot.js`:", e));

import("./theme.js")
  .then((module) => {
    window.setTheme = module.setTheme;
    window.handleTheme = module.handleTheme;
    window.setFormFields = module.setFormFields;
  })
  .catch(e => console.error("Error importing `theme.js`:", e));

