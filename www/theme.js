export function setTheme(colour, cTheme1, cTheme2, tTheme1, tTheme2, letters) {
    localStorage.setItem("colour", colour);
    localStorage.setItem("cTheme1", cTheme1);
    localStorage.setItem("cTheme2", cTheme2);
    localStorage.setItem("tTheme1", tTheme1);
    localStorage.setItem("tTheme2", tTheme2);
    localStorage.setItem("letters", letters);
}

export function getTheme() {
    let colour = localStorage.getItem("colour") || "light";
    let cTheme1 = localStorage.getItem("cTheme1") || "red";
    let cTheme2 = localStorage.getItem("cTheme2") || "yellow";
    let tTheme1 = localStorage.getItem("tTheme1") || "green";
    let tTheme2 = localStorage.getItem("tTheme2") || "pink";
    let letters = localStorage.getItem("letters") || "true";
    return [colour, cTheme1, cTheme2, tTheme1, tTheme2, letters];
}

export function handleTheme() {
    let [colour, cTheme1, cTheme2, tTheme1, tTheme2, letters] = getTheme();
    
    var cContent1, cContent2, tContent1, tContent2;
    if (letters == "true") {
        cContent1 = "O";
        cContent2 = "X";
        tContent1 = "T";
        tContent2 = "O";
    } else {
        cContent1 = "";
        cContent2 = "";
        tContent1 = "";
        tContent2 = "";
    }

    // connect 4 token colors

    let cTheme1Class = "token-" + cTheme1;
    let cTheme2Class = "token-" + cTheme2;

    let cRed = document.getElementsByClassName("red-filled");
    for (var i = 0; i < cRed.length; i++) {
        cRed[i].classList.add(cTheme1Class);
        cRed[i].value = cContent1;
    }
    let cYel = document.getElementsByClassName("yellow-filled");
    for (var i = 0; i < cYel.length; i++) {
        cYel[i].classList.add(cTheme2Class);
        cYel[i].value = cContent2;
    }

    // t/o token colors

    let tTheme1Class = tTheme1 == null ? "token-red" : "token-" + tTheme1;
    let tTheme2Class = tTheme2 == null ? "token-yellow" : "token-" + tTheme2;

    let tTOOT = document.getElementsByClassName("toot-token");
    for (var i = 0; i < tTOOT.length; i++) {
        tTOOT[i].classList.add(tTheme1Class);
        tTOOT[i].value = tContent1;
    }
    let tOTTO = document.getElementsByClassName("otto-token");
    for (var i = 0; i < tOTTO.length; i++) {
        tOTTO[i].classList.add(tTheme2Class);
        tOTTO[i].value = tContent2;
    }

    console.log(colour);
    if (colour == null) {
        document.documentElement.className = "light";
    } else {
        document.documentElement.className = colour;
    }
}

export function setFormFields() {
    let [colour, cTheme1, cTheme2, tTheme1, tTheme2, letters] = getTheme();
    
    let colorThemeRadio = document.getElementById("colourTheme_" + colour);
    colorThemeRadio.checked = true;

    var cTheme1Select = document.getElementById("cTheme1Select");
    cTheme1Select.value = cTheme1;
    var cTheme2Select = document.getElementById("cTheme2Select");
    cTheme2Select.value = cTheme2;
    var tTheme1Select = document.getElementById("tTheme1Select");
    tTheme1Select.value = tTheme1;
    var tTheme2Select = document.getElementById("tTheme2Select");
    tTheme2Select.value = tTheme2;

    if (letters == "true") {
        document.getElementById("accessibility_letters").checked = true;
    }

};