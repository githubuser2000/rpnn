/*interface MapStringNelse<T> extends Map<string, T> {
  [key: string]: T;
}*/
//const MapStringNelse: Map<{[key: string]: string}> = new Map();
var col;
var selectedSpaltenMany1: Map<number, HTMLElement> = new Map<number, HTMLElement>();
var selectedSpaltenMany2: Map<number,string> = new Map<number,string>();
var labelstyle: string = "white-space: nowrap;font-size: 100%;";
var labelstylekl: string = "white-space: nowrap;font-size: 80%;color: grey;";
var tdStyleWhiteSpace: string = "nowrap";
var tdStyleFontSize: string = "100%";
var tdStyleFontSizeKl:string = "80%";
var tdStyleColorKl:string = "grey";
var Enume1 : Set<number> = new Set([0, 1, 3, 4, 5, 6]);
var mapMapMapTags: Map<number, string[]> = new Map<number, string[]>();
var chks1: HTMLCollectionOf<HTMLInputElement>;
var chks2: string[][];
var spaltenTags: Array<Array<number>>;
var spalten4spaltenTags: Map<number, HTMLTableCellElement[]>;
var Achks: HTMLCollectionOf<HTMLInputElement>;
let tdClasses: HTMLCollectionOf<HTMLTableCellElement> = document.getElementsByClassName("z_0") as HTMLCollectionOf<HTMLTableCellElement>;
var mapMapMap: Map<string,Map<string, number>> = new Map();
var insertnull: string;
var erlaubteZeilen: Set<number> = new Set();
var starPolygons: StarPolygon[] = [];
var tAble:  HTMLTableElement;
var TRs: HTMLCollectionOf<HTMLTableRowElement>;
var TDs: HTMLCollectionOf<HTMLTableCellElement>;
var sPolygon: StarPolygon;
var gfPolygon: gleichfPolygon;
var polyg1: string;
var polyg2: string;
//var chkClassNameBySpaltenNr: Map<number, string[][]> = new Map<number, string[][]>();
//var ifDrawSpoly: Set<number>;
//var ifDrawgfPoly: Set<number>;
let pSize: number = 120;
var i2: number;
var enumi: Set<number>;
function animateAllPolygons() {
    for (var i: number=0; i < starPolygons.length; i++) {
        starPolygons[i].animate();
    }
}

async function checksum(object: any) {
  // Konvertiert das Objekt in einen String
  const jsonString = JSON.stringify(object);

  // Konvertiert den JSON-String in ein Uint8Array
  const encoder = new TextEncoder();
  const data = encoder.encode(jsonString);

  // Erstellt eine Checksumme mit dem SHA-256-Algorithmus
  const hashBuffer = await crypto.subtle.digest("SHA-256", data);

  // Gibt die Checksumme im Hexadezimalformat zurück
  const hashArray = Array.from(new Uint8Array(hashBuffer));
  return hashArray.map(b => b.toString(16).padStart(2, "0")).join("");
}


function getRandomColor(): string {
  // Generiere eine zufällige Hex-Farbe (#RRGGBB)
  const letters = "0123456789ABCDEF";
  let color = "#";
  for (let i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

function darkenColor(color: string): string {
  // Wandele die Hex-Farbe in eine RGB-Farbe um
  const hex = color.slice(1); // Entferne das #-Zeichen am Anfang
  const r = parseInt(hex.substring(0, 2), 16);
  const g = parseInt(hex.substring(2, 4), 16);
  const b = parseInt(hex.substring(4, 6), 16);

  // Verringere die RGB-Werte, um die Farbe zu verdunkeln
  const darkerR = Math.max(0, Math.round(r * 0.5));
  const darkerG = Math.max(0, Math.round(g * 0.5));
  const darkerB = Math.max(0, Math.round(b * 0.5));

  // Wandle die verdunkelten RGB-Werte zurück in eine Hex-Farbe um
  const darkerHex =
    "#" +
    ("00" + darkerR.toString(16)).slice(-2) +
    ("00" + darkerG.toString(16)).slice(-2) +
    ("00" + darkerB.toString(16)).slice(-2);

  return darkerHex;
}

class gleichfPolygon {
    private canvas: HTMLCanvasElement;
    private context: CanvasRenderingContext2D;

    constructor(size: number = 100, farbe: string = 'black') {
        this.canvas = document.createElement('canvas') as HTMLCanvasElement;
        this.canvas.height = size;
        this.canvas.width = size;
        this.context = this.canvas.getContext('2d') as CanvasRenderingContext2D;
        this.context.strokeStyle = farbe;
        this.context.globalAlpha = 0.6;
    }

    drawPolygon(n: number, centerX: number, centerY: number, radius: number, startAngle: number = 0, blurVar: number = 1) {
        if (n < 2) {
            // Koordinaten des Punkts
            const x = centerX;
            const y = centerY;
            var radius1: number = 1;

            // Größe des Punkts
            const size = 2;
            this.context.fillStyle = "white";
            this.context.strokeStyle = "white";
            this.context.beginPath();
            //this.context.fillRect(x - size, y - size, size * 2, size * 2);
            this.context.arc(x, y, radius1, 0, size * Math.PI);
            this.context.filter = "blur("+blurVar.toString()+"px)";
            this.context.closePath();
            this.context.stroke();
            return this.canvas.toDataURL();
        }

        let angleStep = Math.PI * 2 / n;
        this.context.beginPath();

        for (let i = 0; i <= n; i++) {
            let angle = i * angleStep + startAngle;
            let x = centerX + radius * Math.cos(angle);
            let y = centerY + radius * Math.sin(angle);
            if (i === 0) {
                this.context.moveTo(x, y);
            } else {
                this.context.lineTo(x, y);
            }
        }

        this.context.filter = "blur("+blurVar.toString()+"px)";
        this.context.closePath();
        this.context.stroke();
        return this.canvas.toDataURL();
    }
}

class StarPolygon {
    private canvas: HTMLCanvasElement;
    private context: CanvasRenderingContext2D;
    private rotationAngle: number;
    private rotationSpeed: number;

    constructor(size: number = 100, farbe: string = 'black') {
        this.canvas = document.createElement('canvas') as HTMLCanvasElement;
        this.canvas.height = size;
        this.canvas.width = size;
        this.context = this.canvas.getContext('2d') as CanvasRenderingContext2D;
        this.context.strokeStyle = farbe;
        this.rotationAngle = 10;
        this.rotationSpeed = 0.1;
        this.context.globalAlpha = 0.6;
    }
    animate() {
        // Vor jedem Frame die vorherige Zeichnung löschen
        this.context.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Neue Transformation anwenden
        this.context.save();
        this.context.translate(this.canvas.width / 2, this.canvas.height / 2);
        this.context.rotate(this.rotationAngle);

        // Hier kannst du deine Zeichenoperationen durchführen
        this.context.beginPath();
        // ...

        // Zeichnen
        this.context.stroke();

        // Transformation zurücksetzen
        this.context.restore();

        // Rotationswinkel aktualisieren
        this.rotationAngle += this.rotationSpeed;

        // Nächsten Frame anfordern
        requestAnimationFrame(this.animate);
    }
    drawStarPolygon(n: number, centerX: number, centerY: number, radius: number, startAngle: number = 0, blurVar: number = 1) {
        if (n < 5) {
            console.log("Cannot draw a star polygon with less than 5 points");
            return this.canvas.toDataURL();
        }

        let angleStep = Math.PI * 2 / n;
        this.context.beginPath();

        let skip = Math.floor(n/Math.PI**(1/Math.PI));

        for (let i = 0; i < n; i++) {
            let angle1 = i * angleStep + startAngle;
            let x1 = centerX + radius * Math.cos(angle1);
            let y1 = centerY + radius * Math.sin(angle1);

            let j = (i + skip) % n;
            let angle2 = j * angleStep + startAngle;
            let x2 = centerX + radius * Math.cos(angle2);
            let y2 = centerY + radius * Math.sin(angle2);

            this.context.moveTo(x1, y1);
            this.context.lineTo(x2, y2);
        }

        this.context.filter = "blur("+blurVar.toString()+"px)";
        this.context.stroke();
        return this.canvas.toDataURL();
    }
}
/*
function subFkt1_PolyTpes_oldAndToReplaceThisFkt(Enums1: Array<number> | Set<number> | HTMLCollectionOf<any>): number[] {
  var Enums: Set<ST> | Array<number> = new Set(Enums1);
  var abzug: number[] = [];
  if (Enums.has(ST.gebrRat) && !Enums.has(ST.sternPolygon)) abzug.push(ST.sternPolygon);
  if (Enums.has(ST.gebrRat) && !Enums.has(ST.gleichfoermigesPolygon)) abzug.push(ST.gleichfoermigesPolygon);
  if (Enums.has(ST.gleichfoermigesPolygon) && !Enums.has(ST.sternPolygon)) abzug.push(ST.sternPolygon);
  if (Enums.has(ST.gleichfoermigesPolygon) && !Enums.has(ST.gebrRat)) abzug.push(ST.gebrRat);
  if (Enums.has(ST.sternPolygon) && !Enums.has(ST.gleichfoermigesPolygon)) abzug.push(ST.gleichfoermigesPolygon);
  if (Enums.has(ST.sternPolygon) && !Enums.has(ST.gebrRat)) abzug.push(ST.gebrRat);
  if (Enums.has(ST.galaxie) && !Enums.has(ST.universum)) abzug.push(ST.universum);
  if (Enums.has(ST.galaxie) && !Enums.has(ST.keinParaOdMetaP)) abzug.push(ST.keinParaOdMetaP);
  if (Enums.has(ST.universum) && !Enums.has(ST.galaxie)) abzug.push(ST.galaxie);
  if (Enums.has(ST.universum) && !Enums.has(ST.keinParaOdMetaP)) abzug.push(ST.keinParaOdMetaP);
  if (Enums.has(ST.keinParaOdMetaP) && !Enums.has(ST.galaxie)) abzug.push(ST.galaxie);
  if (Enums.has(ST.keinParaOdMetaP) && !Enums.has(ST.universum)) abzug.push(ST.universum);
  var Enume : Set<number>= new Set([...Enums, ...Enume1]);
  for (var i = 0; i < abzug.length; i++) Enume.delete(abzug[i]);
  return Array.from(Enume);
}
*/

enum ST {
  sternPolygon = 0,
  gleichfoermigesPolygon = 1,
  keinPolygon = 2,
  galaxie = 3,
  universum = 4,
  keinParaOdMetaP = 5,
  gebrRat = 6,
}

var merkeEnumTags: Set<ST> = new Set<ST>([ST.sternPolygon, ST.gleichfoermigesPolygon, ST.gebrRat, ST.galaxie, ST.universum, ST.keinParaOdMetaP]);
var vergangenheitEnumTagsZahlenArt: Set<ST> = new Set<ST>([ST.sternPolygon, ST.gleichfoermigesPolygon, ST.gebrRat]);
var vergangenheitEnumTagsUniOrGal: Set<ST> = new Set<ST>([ST.galaxie, ST.universum, ST.keinParaOdMetaP]);

function subFkt1_PolyTpes2(Enums1: Array<number> | Set<number> | HTMLCollectionOf<any>): ST[] {
  // WICHTIG: eigentlich müsste man aus Typ ST[][]  Typ TS[][][] machen, wo noch mal für jede Spalte einzeln unterschieden wird: Denn dann kann richtiger für die eine Checkbox dieser Spalten bestimmt werden, ob wann diese ausgegraut werden soll, wann dessen Spalten ausgegraut sind. So ist es wie jetzt mehr falsch, aber es ist okay, weil es auch eine eigene Art von Richtigkeit dabei besitzt, weshalb ich das mal so toleriere, denn das hat eine abstraktere Richtigkeit, die okay sein kann!
  var Enums: Set<ST> = new Set<ST>(Enums1);
  //var abzug: number[] = [];
  /*
  if ((Enums.has(ST.sternPolygon) || Enums.has(ST.gleichfoermigesPolygon) || Enums.has(ST.gebrRat)) && merkeEnumTags[0].size > 0) {
    merkeEnumTags[0] = new Set<ST>();

  } else if ((Enums.has(ST.galaxie) || Enums.has(ST.universum) || Enums.has(ST.keinParaOdMetaP)) && merkeEnumTags[1].size > 0 ) {
    merkeEnumTags[1] = new Set<ST>();
  }*/
  var a: Set<ST> = Enums;
  var b: Set<ST> = new Set<ST>([ST.sternPolygon, ST.gleichfoermigesPolygon, ST.gebrRat]);
  var zahlenArt: Set<ST> = new Set([...a].filter(i => b.has(i)));
  //a = Enums;
  b = new Set<ST>([ST.galaxie, ST.universum, ST.keinParaOdMetaP]);
  var uniOrGal: Set<ST> = new Set([...a].filter(i => b.has(i)));
  if (zahlenArt.size == 0)
    zahlenArt = vergangenheitEnumTagsZahlenArt;
  if (uniOrGal.size == 0)
    uniOrGal = vergangenheitEnumTagsUniOrGal;
  /*
  if (Enums.has(ST.sternPolygon) || Enums.has(ST.gleichfoermigesPolygon) || Enums.has(ST.gebrRat)) {
      Enums.
  }
  if (Enums.has(ST.galaxie) || Enums.has(ST.universum) || Enums.has(ST.keinParaOdMetaP)) {
  }*/
  //var Enums2: Set<ST> = new Set([...Enums, ...merkeEnumTags);
  //console.log(Enums);
  //console.log(merkeEnumTags[0]);
  //console.log(merkeEnumTags[1]);
  /*
  if (Enums.has(ST.sternPolygon) || Enums.has(ST.gleichfoermigesPolygon) || Enums.has(ST.gebrRat)) {
    merkeEnumTags[0] = Enums;
    merkeEnumTags[1] = new Set<ST>();

  } else if (Enums.has(ST.galaxie) || Enums.has(ST.universum) || Enums.has(ST.keinParaOdMetaP)) {
    merkeEnumTags[0] = new Set<ST>();
    merkeEnumTags[1] = Enums;
  }*/
  //console.log(merkeEnumTags);
  //console.log("letzte 1");
  /*
  if (Enums.has(ST.gebrRat) && !Enums.has(ST.sternPolygon)) abzug.push(ST.sternPolygon);
  if (Enums.has(ST.gebrRat) && !Enums.has(ST.gleichfoermigesPolygon)) abzug.push(ST.gleichfoermigesPolygon);
  if (Enums.has(ST.gleichfoermigesPolygon) && !Enums.has(ST.sternPolygon)) abzug.push(ST.sternPolygon);
  if (Enums.has(ST.gleichfoermigesPolygon) && !Enums.has(ST.gebrRat)) abzug.push(ST.gebrRat);
  if (Enums.has(ST.sternPolygon) && !Enums.has(ST.gleichfoermigesPolygon)) abzug.push(ST.gleichfoermigesPolygon);
  if (Enums.has(ST.sternPolygon) && !Enums.has(ST.gebrRat)) abzug.push(ST.gebrRat);
  if (Enums.has(ST.galaxie) && !Enums.has(ST.universum)) abzug.push(ST.universum);
  if (Enums.has(ST.galaxie) && !Enums.has(ST.keinParaOdMetaP)) abzug.push(ST.keinParaOdMetaP);
  if (Enums.has(ST.universum) && !Enums.has(ST.galaxie)) abzug.push(ST.galaxie);
  if (Enums.has(ST.universum) && !Enums.has(ST.keinParaOdMetaP)) abzug.push(ST.keinParaOdMetaP);
  if (Enums.has(ST.keinParaOdMetaP) && !Enums.has(ST.galaxie)) abzug.push(ST.galaxie);
  if (Enums.has(ST.keinParaOdMetaP) && !Enums.has(ST.universum)) abzug.push(ST.universum);
  var Enume : Set<ST>= new Set([...Enums, ...Enume1]);
  for (var i = 0; i < abzug.length; i++) Enume.delete(abzug[i]);
  */
  //console.log(Enums);
  //console.log(abzug);
  vergangenheitEnumTagsZahlenArt = zahlenArt;
  vergangenheitEnumTagsUniOrGal = uniOrGal;
  //console.log(Enums2);
  //console.log("letzte 4");
  return Array.from(new Set([...zahlenArt, ...uniOrGal]));
}



function giveSetOfPolyTypes(Enums1: Array<number> | Set<number> | HTMLCollectionOf<any>): void  {
  //var Enums = subFkt1_PolyTpes1(Enums1);
  var Enums = subFkt1_PolyTpes2(Enums1);
  subFkt3(Enums, SubFkt3SubFkt2bPtr, (unimportantVar: number) => {}, TRs);
}

var SubFkt3SubFkt2bPtr: (i: number, k?: number) => void | Set<number> = function SubFkt3SubFkt2b(i: number, k: number=0): void {
    if (!isNaN(TDs[1].innerHTML.trim())) {
        if (i>4 && i<21) {
            //for (var m: number = 0; m < spalten4spaltenTags[k].length; m++) {
            for (var m: number = 0; m < TDs.length; m++) {
                if (spaltenTags[m].includes(0) || !spaltenTags[m].includes(1)) {
                    //window.alert("yes2");
                    //TDs[m].style.backgroundImage = 'url(' + polyg1 + ')';
                    spalten4spaltenTags[m][i].style.backgroundImage = 'url(' + polyg1 + ')';
                    //TDs[m].style.backgroundRepeat = 'no-repeat';
                    //TDs[m].style.backgroundPosition = 'center';
                    spalten4spaltenTags[m][i].style.backgroundPosition = 'center';
                }
                if (spaltenTags[m].includes(1) && !spaltenTags[m].includes(0)) {
                    //window.alert("yes2");
                    spalten4spaltenTags[m][i].style.backgroundImage = 'url(' + polyg2 + ')';
                    //TDs[m].style.backgroundRepeat = 'no-repeat';
                    spalten4spaltenTags[m][i].style.backgroundPosition = 'center';
                }
            }
        }
        if (i<5) {
            for (var m: number = 0; m < TDs.length; m++) {
                if (true) {
                    //window.alert("yes2");
                    spalten4spaltenTags[m][i].style.backgroundImage = 'url(' + polyg2 + ')';
                    //TDs[m].style.backgroundRepeat = 'no-repeat';
                    spalten4spaltenTags[m][i].style.backgroundPosition = 'center';
                }
            }
        }
    }
}
function returnChangeButtons(number1: number): string {
  var number = number1.toString()
  return (
    '<label style="white-space: nowrap;font-size: 100%;"><input type="radio" class="neuErlauben" name="zeilenDazuOrWeg' +
    number +
    '" onchange="" checked="true">neu sichtbar</label><label style="white-space: normal;">&nbsp; </label><label style="white-space: nowrap;font-size: 100%;"><input type="radio" class="neuHinfort" name="zeilenDazuOrWeg' +
    number +
    '" onchange="">neu unsichtbar</label><label style="white-space: normal;">&nbsp; </label><label style="white-space: nowrap;font-size: 100%;"><input type="radio" class="dazuErlauben" name="zeilenDazuOrWeg' +
    number +
    '" onchange="">zusätzlich sichtbar</label><label style="white-space: normal;">&nbsp; </label><label style="white-space: nowrap;font-size: 100%;"><input type="radio" class="dazuEinschraenkend" name="zeilenDazuOrWeg' +
    number +
    '">zusätzlich eingeschränkt</label><label style="white-space: normal;">&nbsp; </label><label style="white-space: nowrap;font-size: 100%;"><input type="radio" class="dazuHinfort" name="zeilenDazuOrWeg' +
    number +
    '" onchange="">zusätzlich unsichtbar</label>'
  );
}



window.onload = function () {
  let div: HTMLDivElement = document.createElement("div");
  let div2: HTMLDivElement = document.createElement("div");
  tAble = document.getElementById("bigtable") as HTMLTableElement;
  TRs = tAble.rows;

  div.className = "headingsDiv";
  /*
    sternPolygon = 0
    gleichfoermigesPolygon = 1
    keinPolygon = 2
    galaxie = 3
    universum = 4
*/
  document.body.insertBefore(div, document.getElementById("bigtable"));

  let chk_spalten: string =
    '<fieldset><label style="white-space: nowrap;"><input type="radio" id="spaltenWahl" name="spaltOrZeilWahl" onchange="toggleChkSpalten(this);" checked="true">Spalten (Einheiten [9]) wählen</label> <label style="white-space: nowrap;"><input type="radio" id="zeilenWahl" name="spaltOrZeilWahl" onchange="toggleChkSpalten(this);">Zeilen, welche ja nein, (6,13,14,15) (wenig: 7,8,10,12)</label> <label style="white-space: nowrap;"><input type="radio" id="keinsWahl" name="spaltOrZeilWahl" onchange="toggleChkSpalten(this);">frei machen zur Tabellenansicht <!-- | Lädt schneller mit Firefox statt Chrome --> </label></fieldset>';
  let radio_tags: string =
    '<fieldset><label style="white-space: nowrap;"><input type="radio" id="galaxieuniversum" name="galaxieuniversum" onchange="disEnAbleChks([3,4,5]);" checked="true">alle Zahlengrößenordnungen</label> <label style="white-space: nowrap;"><input type="radio" id="planet" name="galaxieuniversum" onchange="disEnAbleChks([5]);">alles andere als 13,15, ggf. jeweils mit 14</label> <label style="white-space: nowrap;"><input type="radio" id="galaxie" name="galaxieuniversum" onchange="disEnAbleChks([3]);">Himmelskörper um schwarzes Loch (13), z.B. eine Galaxie (14)</label> <label style="white-space: nowrap;"><input type="radio" id="universum" name="galaxieuniversum" onchange="disEnAbleChks([4]);">Universum (15)</label></fieldset><fieldset><label style="white-space: nowrap;"><input type="radio" id="sternpolygongleichfoermigespolygon" name="sternpolygongleichfoermigespolygon" onchange="disEnAbleChks([0,1,6]);" checked="true">Sternpolygon und gleichförmiges Polygon und gebrochen-rational</label> <label style="white-space: nowrap;"><input type="radio" id="sternpolygon" name="sternpolygongleichfoermigespolygon" onchange="disEnAbleChks([0]);">Sternpolygon (n)</label> <label style="white-space: nowrap;"><input type="radio" id="gleichfoermigespolygon" name="sternpolygongleichfoermigespolygon" onchange="disEnAbleChks([1]);">gleichförmiges Polygon (1/n)</label> <label style="white-space: nowrap;"><input type="radio" id="gebrrat" name="sternpolygongleichfoermigespolygon" onchange="disEnAbleChks([6]);">gebrochen-rational (m/n)</label></fieldset>';
  div.innerHTML = chk_spalten;
  /*tdClasses = []
for (i = 0; i < tdClasses1.length; i++)
	if (tdClasses1[i].className.includes("z_0"))
		tdClasses.push(tdClasses1[i]);*/
  //let p1map = new Map();
  //let p2map = new Map();
  // str: string = "",
  //let p1Bmap = new Map();
  var str3: string = "";
  var trStyles: Array<string> = [];

  spalten4spaltenTags = new Map();
  for (var i: number = 0; i < TRs.length; i++) {
    trStyles.push(TRs[i].style.cssText);
    var TDs: HTMLCollectionOf<HTMLTableCellElement> = TRs[i].cells;
    for (var k: number = 0; k < TDs.length; k++) {
      if (typeof spalten4spaltenTags[k] == "undefined")
        spalten4spaltenTags[k] = new Array();
      spalten4spaltenTags[k].push(TDs[k]);
    }
  }
  // spalten4spaltenTags[k]: Index ist Spalte und Werte sind alle Zeilen dieser Spalte
  /*(async () => {
  const result = await checksum(TRs);
  console.log("Checksum:", result);
  })();
  (async () => {
  const result = await checksum(spalten4spaltenTags);
  console.log("Checksum:", result);
  })();*/

  spaltenTags = []
  var tags: number[];
  for (var i: number = 0; i < tdClasses.length; i++) {
    var name: string = tdClasses[i].className;
    var num1: RegExpMatchArray | null   = name.match(/r_(\d+)/);

    var tags1: RegExpMatchArray | null = name.match(/p4_([\d,]+)/g);
    if (tags1 === null) tags = [];
    else tags = String(tags1).substr(3).split(",").map((str) => parseInt(str, 10));;
    spaltenTags.push(tags);

      /*(async () => {
      const result = await checksum(spaltenTags);
      console.log("Checksum:", result);
      })();*/
    if (num1 != null) {
      //num = num.substring(2,0);
      var num: number = parseInt(num1[1]);
      //let str = num[1];
      //num = i
      var p1a: RegExpMatchArray | null  = name.match(/p1_([^\s])+/g);
      var p2a: RegExpMatchArray | null  = name.match(/p2_([^\s])+/g);
      // p1a und p2a entsprechen 2 Dingen in Tabellen-Zellen-Klassen von: -spalten --ding1a=ding2a,ding2b --ding1b=ding2a
      // aus diesen beiden listen und der Nummer der Spalte und den tags wird etwas gebaut
      // mapMapMap und mapMapMapTags: indexe sind ding1+ding2 und
      // das eine hat als Wert die Tags und das andere die Nummern der Spalten
      // die Spaltennummern sind ints
      if (p1a != null) {
        for (var p1i: number = 0; p1i < p1a.length; p1i++) {
          if (p1a[p1i].includes("p1_")) p1a[p1i] = p1a[p1i].substring(3);
          var p1b : RegExpMatchArray | null = p1a[p1i].match(/[^,]+/g);
          if (p1b != null) {
            for (let p1k: number = 0; p1k < p1b.length; p1k++) {
              var p1: string = p1b[p1k];
              if (typeof mapMapMap[p1] === "undefined") mapMapMap[p1] = new Map();
              if (p2a != null) {
                for (var p2i: number = 0; p2i < p2a.length; p2i++) {
                  if (p2a[p2i].includes("p2_"))
                    p2a[p2i] = p2a[p2i].substring(3);
                  var p2b: RegExpMatchArray | null = p2a[p2i].match(/[^,]+/g);
                  if (p2b != null) {
                    for (var p2k: number = 0; p2k < p2b.length; p2k++) {
                      var p2: string = p2b[p2k];
                      if (p2 != null) {
                        var p3a: RegExpMatchArray | null  = p2.match(/p3_(\d+)_/);
                        if (p3a != null) {
                          var p3b: number = parseInt(p3a[1], 10);
                          var p2: string = p2.substring(p3a[1].length + 4);
                          if (p3b == p1k) {
                            if (p2.length > 0)
                              makeMapsOfHeadLCheckB(p1, p2, num, tags);
                            else makeMapsOfHeadLCheckB(p1, null, num, tags);
                          }
                        }
                      } else makeMapsOfHeadLCheckB(p1, null, num, tags);
                    }
                  }
                }
              } else makeMapsOfHeadLCheckB(p1, null, num, tags);
            }
          }
        }
      }
    }
  }

  /*(async () => {
  const result = await checksum(mapMapMap);
  console.log("Checksum:", result);
  })();*/


  var p1keys: string[] = Object.keys(mapMapMap);
  //var p1Bkeys = Object.keys(p1Bmap);
  //checkboxes = "<span style=\"white-space: nowrap;\"><input type=\"checkbox\" onchange=\"toggleSpalten(\'r_0\');\"><label>Nummererierung</label>";
  var checkboxes : string =
    '<div id="chk_spalten" style="display:none;">' +
    radio_tags +
    '<span style="">';
  for (var i: number = 0; i < p1keys.length; i++) {
    var chk2s: string = "";
    var p2keys: string[] = Object.keys(mapMapMap[p1keys[i]]);
    //console.log("das Array Objekt 1: "+Array)
    //console.log("das Array Objekt 2: "+Array)
    for (var k: number = 0; k < p2keys.length; k++) {
      //console.log(typeof mapMapMap[p1keys[i]][p2keys[k]]);
      //console.log(mapMapMap[p1keys[i]][p2keys[k]]);
      var spaltenNrs: Set<number> = mapMapMap[p1keys[i]][p2keys[k]]
      var mapMapMapSetValue: Set<number> = spaltenNrs;
      /*console.log("das Array Objekt 3: "+Array)
      console.log(mapMapMapSetValue);
      console.log(p1keys[i]);
      console.log(p2keys[k]);*/
      /*
      if ("Liebe" == p2keys[k]) {
          console.log("break 1");
          break;
      }
      if ("Maßnahmen_(39)" == p2keys[k]) {
          console.log("break 2");
          break;
      }*/
      //try {
       //console.log("das Array Objekt A: "+Array)
       //console.log("das Array Objekt A Methode: "+Array.from)
        var numbers: Array<number> = Array.from(mapMapMapSetValue);
      /*} catch {
        var numbers: Array<number> = Array.from(mapMapMapSetValue);
      }*/
      //console.log("das Array Objekt 4: "+Array)
      if (p2keys[k] != null && p2keys[k] != "null") {
        // window.alert(p1keys[i]); '✗Grundstrukturen'
        // window.alert(p2keys[i]); klar
        // window.alert(numbers); // ach einfach die und daraus!
        // window.alert(Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(",")); // diese Zahlen
        //console.log("das Array Objekt 5: "+Array)
        var chk2: string =
          '<label style="' +
          labelstyle +
          '" class="chks c_' +
          Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(",") +
          '" ><input type="checkbox" class="chks c_' +
          Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(",") +
          '" value="' +
          p2keys[k] +
          '" onchange="toggleP2(this,\'' +
          numbers +
          "','" +
          [p1keys[i], p2keys[k]] +
          "');\">" +
          makeSpacesOutOf_(p2keys[k].toString()) +
          '</input></label><label style="white-space: normal;">&nbsp; </label>';
        chk2s += chk2;
        //console.log("das Array Objekt 6: "+Array)
      }
      //console.log("das Array Objekt 7: "+Array)
      //
      // nein, das geht so nicht:
      /*
      for (var numr of spaltenNrs) {
        try {
            chkClassNameBySpaltenNr[numr].push("chks c_ "+Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(","));
        } catch {
            chkClassNameBySpaltenNr[numr] = [];
            chkClassNameBySpaltenNr[numr].push("chks c_ "+Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(","));
            // hat also die Spaltennummer als index und den Klassennamen der zugehörigen checkboxen als werte eines Arrays
        }
      }*/

    }
    if (p1keys[i] === "✗Grundstrukturen") {
      var grunSi: number = i;
      var grunp2Keys: string[] = p2keys;
    }

      /*(async () => {
      const result = await checksum(p1keys);
      console.log("Checksum X:", result);
      })();
      (async () => {
      const result = await checksum(p2keys);
      console.log("Checksum Y:", result);
      })();*/
    if (typeof mapMapMap[p1keys[i]][null] !== "undefined") {
      var numbers: number[]  = Array.from(mapMapMap[p1keys[i]][null]);
      insertnull = "toggleP2(this,'" + numbers.toString() + "','" + [p1keys[i], null] + "');";
    } else {
      insertnull = "";
    }
    var mapsTagsif: string[]  = mapMapMapTags[p1keys[i]][null];
    if (typeof mapsTagsif == "undefined") mapsTagsif = [];
    else mapsTagsif = Array.from(mapMapMapTags[p1keys[i]][null]);
      /*(async () => {
      const result = await checksum(mapMapMapTags);
      console.log("Checksum:", result);
      })();*/

    var checkbox: string =
      '<div class="chksA"><label class="chksA1 c1_' +
      mapsTagsif.join(",") +
      '" style="' +
      labelstyle +
      '"><input class="chksA2" type="checkbox" ' + // class="chks c_' +
      //Array.from(mapMapMap[p1keys[i]][null]).join(",") +
      //'"  value="' +
      ' value="' +
      String(p1keys[i]) +
      '" onchange="toggleP1(\'' +
      String(p1keys[i]) +
      "');" +
      String(insertnull) +
      '">' +
      String(makeSpacesOutOf_(p1keys[i])) +
      "</input></label>" +
      '<div id="' +
      String(p1keys[i]) +
      '" style="display:none;white-space: normal; border-left: 40px solid rgba(0, 0, 0, .0);">' +
      (p1keys[i] === "✗Grundstrukturen"
        ? '<input type="radio" class="grundRadio" id="grundRadioChaos" checked onchange="grundSDivToggle(0)"><label>unübersichtlich</label></input> <input type="radio" class="grundRadio" id="grundRadioOrdnung" onchange="grundSDivToggle(1)"><label>ordentlich</label></input><div id="grundSDiv0">'
        : "") +
      String(chk2s) +
      (p1keys[i] === "✗Grundstrukturen"
        ? '</div><div id="grundSDiv1" style="display:none;"></div>'
        : "") +
      "</div></div>";

    checkboxes += checkbox;
  }
  //console.log(chkClassNameBySpaltenNr);
  var str2: string = checkboxes + "</span></div>";
      /*(async () => {
      const result = await checksum(checkboxes);
      console.log("Checksum A:", result);
      })();*/
  div.innerHTML += str2;
  chks1 = document.getElementsByClassName("chks") as HTMLCollectionOf<HTMLInputElement>;
  chks2 = [];
  for (var i = 0; i < chks1.length; i++) {
    chks2.push(
      String(chks1[i].className.match(/c_([\d,]+)/g))
        .substr(2)
        .split(",")
    );
    //window.alert(chks2[i]);
    //console.log(chks2.length);
  }

  var str4: string =
    '<div id="inputZeilen" style="display:none"><table borders="0" id="table2">';
  var str5: string =
    '<tr><td><label>von bis und Einzelnes: </label></td><td><input typ="text" id="zeilenErlaubtText" value="1-10,12"></input></td><td>' +
    returnChangeButtons(1) +
    '<input onclick="clickZeilenErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str6: string = '<tr><td><label>Vielfacher und Nachbarn: </label></td><td><input typ="text" id="VielfacheErlaubtText" value="10+0+1,7+0"></td><td>' +
    returnChangeButtons(2) +
    '<input onclick="clickVielfacheErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str8: string =
    '<tr><td><label>Potenzen: </label></td><td><input typ="text" id="potenzenErlaubtText" value="3,5"></input></td><td>' +
    returnChangeButtons(3) +
    '<input onclick="clickPotenzenErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str9: string =
    '<tr><td colspan="2"><input type="radio" id="sonneWahl" name="sunmoonplanetblackhole" onchange="" checked="true"><label>Sonne</label><input type="radio" id="mondWahl" name="sunmoonplanetblackhole" onchange=""><label>Mond</label><input type="radio" id="planetWahl" name="sunmoonplanetblackhole" onchange=""><label>Planet</label><input type="radio" id="schwarzeSonneWahl" name="sunmoonplanetblackhole" onchange="" onclick="window.alert(\'Schwarze Sonnen kehren die Originalbedeutung der 3*n ins Gegenteil (Paradigmen in Gegen-Paradigmen; und auch: Meta-Paradigmen, Transzendentalien in Gegen-Meta-Paradigmen, Gegen-Transzendentalien).\');"><label>schwarze Sonne</label></td><td>' +
    returnChangeButtons(4) +
    '<input onclick="clickHimmelskoerperErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str10: string =
    '<tr><td><label>Zählung: </label></td><td><input typ="text" id="zaehlungErlaubtText" value="1,3-4"></input></td><td>' +
    returnChangeButtons(5) +
    '<input onclick="clickZaehlungenErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str11: string =
    '<tr><td><label>Primzahlvielfacher: </label></td><td><input typ="text" id="primVielfache" value="1"></input></td><td>' +
    returnChangeButtons(6) +
    '<input onclick="clickPrimVielfacheErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str12: string =
    '<tr><td colspan="2"><input type="radio" id="proInnen" name="proContra4Richtungen" onchange="" checked="true"><label>pro innen</label><input type="radio" id="proAussen" name="proContra4Richtungen" onchange=""><label>pro außen</label><input type="radio" id="gegenDritte" name="proContra4Richtungen" onchange=""><label>gegen Dritte</label><input type="radio" id="proDritte" name="proContra4Richtungen" onchange="" onclick=""><label>pro Dritte</label></td><td>' +
    returnChangeButtons(7) +
    '<input onclick="clickPrimRichtungenErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str13: string =
    '<tr><td><label>Primzahlkreuzradius: </label></td><td><input typ="text" id="primZahlKreuzRadius" value="1"></input></td><td>' +
    returnChangeButtons(8) +
    '<input onclick="clickPrimZahlKreuzRadiusErlaubenUsw();" type="submit" value="auswählen"></td></tr>';
  var str7: string = "</table></div>";
  div.innerHTML +=
    str4 + str5 + str6 + str8 + str9 + str10 + str11 + str12 + str13 + str7;
  // Spaltenreihenfolge
  var tableHeadline: HTMLCollectionOf<HTMLTableCellElement>= document.getElementById("bigtable").rows[0].cells;
  for (var u: number = 0; u < tableHeadline.length; u++) {
    tableHeadline[ u].innerHTML =
      '<select id="hselec_' +
      u.toString() +
      '" value="' +
      u.toString() +
      '" onchange="headingSelected(this, ' +
      u.toString() +
      ');"></select>' +
      tableHeadline[u].innerHTML;
  }
  toggleChkSpalten();

  var tabelle: HTMLTableElement = document.getElementById("bigtable") as HTMLTableElement;
  var tds: HTMLCollectionOf<HTMLTableCellElement>  = tabelle.cells;
  /*
  for (var i = 0; i < tds.length; i++) {
    text = tds[i];
    text.innerHTML = [
      "<label>",
      //text.innerHTML.replaceAll(") | ", ") | <br>").trim(),
      text.innerHTML,
      "</label>",
    ].join("");
  }
  */

  var trs: HTMLCollectionOf<HTMLTableRowElement> = tabelle.rows;
  var tdsHeadlines: HTMLCollectionOf<HTMLTableCellElement> = trs[0].cells;
  var classnames: string[] = [];
  for (var i: number = 0; i < tdsHeadlines.length; i++)
    classnames.push(tdsHeadlines[i].className);
  for (var k: number = 1; k < trs.length; k++) {
    tds = trs[k].cells;
    for (var i: number = 0; i < tds.length; i++)
      tds[i].className = classnames[i].replace("z_0", "z_" + tds[1].innerHTML);
  }

  for (var k: number = 0; k < trs.length; k++) {
    var tds: HTMLCollectionOf<HTMLTableCellElement>  = trs[k].cells;
    tds[0].style.cssText += "display:none;";
    for (var i: number = 1; i < tds.length; i++)
      tds[i].style.cssText = [
        tds[i].style.cssText,
        "display:none;",
        trs[k].style.cssText,
      ].join("");
    tds[0].style.textAlign = "center";
    tds[1].style.textAlign = "center";
    trs[k].style.cssText = "";
  }
  /*
  for (var k = 0; k < trs.length; k++) {
    tds = trs[k].cells;
    for (var i = 2; i < tds.length; i++)
      tds[i].style.cssText = tds[1].style.cssText;
  }*/

  var inputs: HTMLCollectionOf<HTMLInputElement> = document.getElementsByTagName("input");
  var checkbox_i: Array<any> = [];
  for (var i: number = 0; i < inputs.length; i++) {
    if (inputs[i].type == "checkbox") checkbox_i.push(i);
    if (checkbox_i.length > 1) i = inputs.length;
  }
  const queryString: string = window.location.search;
  const urlParams: URLSearchParams = new URLSearchParams(queryString);
  const ifpreselect: string  = urlParams.get("preselect");
  if (ifpreselect != "nothing" && checkbox_i != null && this != null) {
    inputs[checkbox_i[1]].checked = true;
    inputs[checkbox_i[1]].onchange(this);
  }

  //var sheets: StyleSheetList = document.styleSheets;
  //var sheet, rules, rule;
  document.body.style.display = "initial";
  //animateAllPolygons();
  /*
  for (var i: number = 0, iLen = sheets.length; i < iLen; i++) {
    sheet = sheets[i];
    // W3C model
    if (sheet.cssRules) {
      rules = sheet.cssRules;

      for (var j: number = 0, jLen = rules.length; j < jLen; j++) {
        rule = rules[j];
        if (rule.cssText == "body { display: none; }") {
          //window.alert(rule.cssText);
          if (typeof sheet.deleteRule == "function") {
            sheet.deleteRule(rule);
            jLen = rules.length;
          }
          //window.alert(rule.selectorText);
        }
      }
    }
  }*/
  document.getElementById("grundSDiv1").innerHTML =
    document.getElementById("grundstrukturenDiv").innerHTML;
  //window.alert(String(checkbox_i.length));
  // ✗Grundstrukturen
  // chksss = chks1 + Achks;
  var Achks: HTMLCollectionOf<HTMLInputElement> = document.getElementsByClassName("chksA2");
  var dinge: string[] = [
    "✗Grundstrukturen",
    "✗Universum",
    "Geist__(15)",
    "Strukturalien_bzw_Meta-Paradigmen_bzw_Transzendentalien_(15)",
  ];
  if (ifpreselect != "no_universal" && ifpreselect != "nothing") {
    var dinge2: Array<HTMLCollectionOf<HTMLInputElement>> = [Achks, Achks, chks1, chks1];
    for (var x: number = 0; x < dinge.length; x++) {
      var checkx: HTMLInputElement[] = [];
      for (var k: number = 0; k < dinge2[x].length; k++)
        if (dinge2[x][k].value == dinge[x]) checkx.push(dinge2[x][k]);
      if (checkx.length > 0) {
        checkx[0].checked = true;
        checkx[0].onchange(this);
      }
    }
  }
  /*copyClassNameToOrderedGrunstruk(
    mapMapMap,
    mapMapMapTags,
    p1keys,
    p2keys,
    grunSi,
    grunp2Keys
  );*/
  //window.alert(TRs.length);
  //var ueberschrift: string;
  //const regex1: RegExp = /(?<!\() n (?!\d|\/)/;
  /*
  const regex2: RegExp = /(?<! )1\/n|(?<!\()1\/n(?!\))/;
  const regex3: RegExp = /(?<! )1\/|(?<!\()1\/(?!\))/;
  const regex4 = /\(\s*\d+\s*\)|\s+\d+\s+|\s+\d+$/;
  const regex5 = /\(\s*1\/\d+\s*\)|\s+1\/\d+\s+|\s+1\/\d+$/;
  const regex6 = /[a-zA-Z\(]+\d+$|[a-zA-Z\(]+\d+[^\d]+/;
  */
  //ifDrawgfPoly =  new Set();
  //ifDrawSpoly =  new Set();
  giveSetOfPolyTypes([1]);
  giveSetOfPolyTypes([0]);
  /*ifDrawgfPoly = new Set([...ifDrawgfPoly].filter((x) => !ifDrawSpoly.has(x)));
  ifDrawSpoly = new Set([...ifDrawSpoly].filter((x) => !ifDrawgfPoly.has(x)));*/
  /*
  for (var i: number = 0; i < TRs.length; i++) {
        TDs = TRs[i].cells as HTMLCollectionOf<HTMLTableCellElement>;
        for (var k: number = 0; k < TDs.length; k++) {
          ueberschrift = TDs[k].innerHTML
          if (i==0 && (ueberschrift.includes('eziproke') || ueberschrift.includes('gleichförm') || regex2.test(ueberschrift)  || regex3.test(ueberschrift) || regex5.test(ueberschrift)) && !((ueberschrift.includes('ternpolygon') && !ueberschrift.includes('nicht-Sternpolygon') ) || ueberschrift.includes(' n ') || ueberschrift.trim().slice(-2) === " n" || ueberschrift.includes('(n)'))) ifDrawgfPoly.add(k);
          else if (i==0 && (ueberschrift.includes('ternpolygon') || (regex6.test(ueberschrift.replace(/\s/g, "").trim()) && isNaN(ueberschrift.replace(/\s/g, ""))) || ueberschrift.includes(' n ') || ueberschrift.trim().slice(-2) === " n" || ueberschrift.includes('(n)') && regex4.test(ueberschrift.trim())) && !(ueberschrift.includes('1/') || ueberschrift.includes('gleichförm') || regex2.test(ueberschrift)  || regex3.test(ueberschrift) || regex5.test(ueberschrift))) ifDrawSpoly.add(k);
        }


        if (i>4 && i<21) {
            if (!isNaN(TDs[1].innerHTML.trim())) {
                    i2 = parseInt(TDs[1].innerHTML.trim())
                    //window.alert(TDs[1].innerHTML);
                    sPolygon = new StarPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                    polyg1 = sPolygon.drawStarPolygon(i, pSize, pSize, 25);
                    starPolygons.push(sPolygon);
                    //sPolygon.animate();
                    gfPolygon = new gleichfPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                    polyg2 = gfPolygon.drawPolygon(i, pSize, pSize, 14);
                    for (var k: number = 0; k < TDs.length; k++) {
                        if ( ifDrawSpoly.has(k)) {
                                //window.alert("yes2");
                                TDs[k].style.backgroundImage = 'url(' + polyg1 + ')';
                                //TDs[k].style.backgroundRepeat = 'no-repeat';
                                TDs[k].style.backgroundPosition = 'center';
                        }
                        if ( ifDrawgfPoly.has(k)) {
                                //window.alert("yes2");
                                TDs[k].style.backgroundImage = 'url(' + polyg2 + ')';
                                //TDs[k].style.backgroundRepeat = 'no-repeat';
                                TDs[k].style.backgroundPosition = 'center';
                        }}

            //}

            }

        }
        if (i<5) {
            if (!isNaN(TDs[1].innerHTML.trim())) {
                    //window.alert(TDs[1].innerHTML);
                    i2 = parseInt(TDs[1].innerHTML.trim())
                    gfPolygon = new gleichfPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                    polyg2= gfPolygon.drawPolygon(i, pSize, pSize, 15);
                    for (var k: number = 0; k < TDs.length; k++) {
                            if ( ifDrawSpoly.has(k) || ifDrawgfPoly.has(k)) {
                                //window.alert("yes2");
                                TDs[k].style.backgroundImage = 'url(' + polyg2 + ')';
                                //TDs[k].style.backgroundRepeat = 'no-repeat';
                                TDs[k].style.backgroundPosition = 'center';
                        }}

            //}

            }

        }
  }
  */
  const backgr: string | null  = urlParams.get("background");
  const randomColor = getRandomColor();
  const darkenedColor = darkenColor(randomColor);
  if (backgr === "710" || backgr === "107") {
    sPolygon = new StarPolygon(250, darkenedColor);
    polyg1 = sPolygon.drawStarPolygon(7, 125, 125, 36, Math.PI/14, 3);
    polyg2 = sPolygon.drawStarPolygon(10, 125, 125, 130, Math.PI/2, 5);
    polyg2 = sPolygon.drawStarPolygon(10, 125, 125, 130, Math.PI/2, 5);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg2 + ')';
  }
  else if (backgr === "7") {
    sPolygon = new StarPolygon(250, darkenedColor);
    polyg1 = sPolygon.drawStarPolygon(7, 125, 125, 130, Math.PI/14, 3);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg1 + ')';
  }
  else if (backgr === "10") {
    sPolygon = new StarPolygon(250, darkenedColor);
    polyg2 = sPolygon.drawStarPolygon(10, 125, 125, 130, Math.PI/2, 5);
    polyg2 = sPolygon.drawStarPolygon(10, 125, 125, 130, Math.PI/2, 5);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg2 + ')';
  }
  else if (backgr === "1pro5") {
    gfPolygon = new gleichfPolygon(250, darkenedColor);
    polyg2 = gfPolygon.drawPolygon(5, 125, 125, 100, 0, 5);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg2 + ')';
  }
  else if (backgr === "1pro6") {
    gfPolygon = new gleichfPolygon(250, darkenedColor);
    polyg2 = gfPolygon.drawPolygon(6, 125, 125, 100, 0, 5);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg2 + ')';
  }
  else if (backgr === "1pro5m6" || backgr == "1pro6m5") {
    gfPolygon = new gleichfPolygon(250, darkenedColor);
    polyg2 = gfPolygon.drawPolygon(5, 125, 125, 80, 0, 5);
    polyg2 = gfPolygon.drawPolygon(6, 125, 125, 100, 0, 5);
    polyg2 = gfPolygon.drawPolygon(100, 125, 125, 110, 0, 5);
    document.body.style.backgroundAttachment = "fixed"
    document.body.style.backgroundImage = 'url(' + polyg2 + ')';
  }
  //var keys1: number[] = chkClassNameBySpaltenNr.keys();
  // nein, das geht so nicht:
  /*
  var ByChkClassNamesToGetSpaltenNr: Map<string, Set<number>> = new Map<string, Set<number>>();
  for (var i=0; i < chkClassNameBySpaltenNr.size;i++ ) {
   //chkClassNameBySpaltenNr[numr].push("chks c_ "+Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(","));
    for (var k=0; k < chkClassNameBySpaltenNr[i].length ;k++) {
      try {
        ByChkClassNamesToGetSpaltenNr[chkClassNameBySpaltenNr[i][k]].add(i)
      } catch {
        ByChkClassNamesToGetSpaltenNr[chkClassNameBySpaltenNr[i][k]] = new Set<number>()
        ByChkClassNamesToGetSpaltenNr[chkClassNameBySpaltenNr[i][k]].add(i)
      }
    }
  }
  */
};

function makeMapsOfHeadLCheckB(p1: string, p2: string | null, num: string | number, tags: any): void {
  try {
    mapMapMap[p1][p2].add(num);
  } catch(e:Exception){
    mapMapMap[p1][p2] = new Set();
    mapMapMap[p1][p2].add(num);
  }
  if (typeof mapMapMapTags[p1] === "undefined") mapMapMapTags[p1] = [];
  if (!(typeof mapMapMapTags[p1][p2] instanceof Set))
    mapMapMapTags[p1][p2] = new Set();
  if (typeof tags != "undefined" && tags != "null") {
    //if (mapMapMapTags[p1][p2].size > 0) { console.log(mapMapMapTags[p1][p2]);var bla = true; } else var bla = false;
    mapMapMapTags[p1][p2] = new Set([...mapMapMapTags[p1][p2], ...tags]);
    //if (bla) console.log(mapMapMapTags[p1][p2]);
    //if (bla) console.log("2 davor");

  }
}

function disEnAbleChks(Enums1: Array<number> | Set<number> | HTMLCollectionOf<any>) {
  //var Enums: number[] = subFkt1_PolyTpes1(Enums1);
  var Enums: number[] = subFkt1_PolyTpes2(Enums1);
  subFkt3(Enums, SubFkt3SubFkt1Ptr2, SubFkt3SubFkt2Ptr2, spaltenTags)
  // weg kommentiert, weil die Fkt fehlerhaft funktioniert und das erst mal weniger wichtig is
  // in der Fkt steht, wie der Fehler ist. Es werden oft nicht die richtigen Checkboxen deaktiviert und aktiviert
  //Enums = subFkt1_PolyTpes2(Enums1);
  subFkt3(Enums, SubFkt3SubFkt1Ptr, SubFkt3SubFkt2Ptr, chks2);

  // nein, das geht so nicht:
  /*
  for (var [key, value] of ByChkClassNamesToGetSpaltenNr.entries()) {
    console.log(`${key}: ${value}`);
    var flag = true;
    for (var j=0; j < value.size; j++) {
        if (spalten4spaltenTags[j][0].style.opacity != "1")
            flag = false;

    }
    var chkX = chks1.getElementsByClassName(key)[0];
    if (!flag) {
      //chkX.style.opacy = "0.4";
      //chkX.style.fontSize = "80%";
    } else {
      //chkX.style.opacy = "1";
      //chkX.style.fontSize = "100%";
    }
  }
  */
 // für später noch verwendbar, aber erst mal nicht:

  var Achks: HTMLCollectionOf<HTMLInputElement> = document.getElementsByClassName("chksA") as HTMLCollectionOf<HTMLInputElement>;
  var Bchks: HTMLCollectionOf<HTMLInputElement>;
  for (var i: number = 0; i < Achks.length; i++) {
    Bchks = Achks[i]
      .getElementsByTagName("div")[0]
      .getElementsByTagName("input");
    var deakAmount: number = 0;
    for (var k: number = 0; k < Bchks.length; k++) if (Bchks[k].disabled) deakAmount++;
    if (deakAmount == Bchks.length && deakAmount != 0) {
      Achks[i].getElementsByTagName("label")[0].style.fontSize =
        tdStyleFontSizeKl;
      Achks[i].getElementsByTagName("label")[0].style.color = tdStyleColorKl;
      Achks[i].getElementsByTagName("label")[0].style.whiteSpace =
        tdStyleWhiteSpace;
    } else {
      Achks[i].getElementsByTagName("label")[0].style.fontSize =
        tdStyleFontSize;
      Achks[i].getElementsByTagName("label")[0].style.color = "";
      Achks[i].getElementsByTagName("label")[0].style.whiteSpace =
        tdStyleWhiteSpace;
    }
  }
  var chksA1label: HTMLCollectionOf<HTMLInputElement> = document.getElementsByClassName("chksA1");
  for (var i: number = 0; i < chksA1label.length; i++) {
    var tagsPerA1Label: RegExpMatchArray  = chksA1label[i].className.match(/c1_([\d,]+)/g);
    if (tagsPerA1Label == null) tagsPerA1Label = [];
    else
      var tagsPerA1Label: RegExpMatchArray  = String(chksA1label[i].className.match(/c1_([\d,]+)/g))
        .substr(3)
        .split(",");
    if (tagsPerA1Label.length != 0) {
      var enumo: Set<number> = new Set();
      for (var k: number = 0; k < tagsPerA1Label.length; k++)
        for (var l:number  = 0; l < Enums.length; l++)
          if (tagsPerA1Label[k] == Enums[l].toString()) enumo.add(Enums[l]);
      if (
        (!enumo.has(0) && !enumo.has(1) && !enumo.has(6)) ||
        (!enumo.has(3) && !enumo.has(4) && !enumo.has(5)) ||
        enumo.size == 0
      ) {
        chksA1label[i].style.fontSize = tdStyleFontSizeKl;
        chksA1label[i].style.color = tdStyleColorKl;
        chksA1label[i].style.whiteSpace = tdStyleWhiteSpace;
      }
    }
  }

}
const alleMonde: number[] = [
  4, 8, 9, 16, 25, 27, 32, 36, 49, 64, 81, 100, 121, 125, 128, 144, 169, 196,
  216, 225, 243, 256, 289, 324, 343, 361, 400, 441, 484, 512, 529, 576, 625,
  676, 729, 784, 841, 900, 961, 1000, 1024,
];

const primZahlen: number[] = [
  2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
  73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
  157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233,
  239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311, 313, 317,
  331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419,
  421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
  509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607,
  613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
  709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811,
  821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911,
  919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997, 1009, 1013, 1019,
  1021,
];

/*
  for (var i: number = 2; i < spaltenTags.length; i++) {
  for (var i: number = 0; i < chks2.length; i++) {
    var enumi: Set<number> = new Set();
    var enumi: Set<number> = new Set();
    for (var k: number = 0; k < spaltenTags[i].length; k++)
    for (var k: number = 0; k < chks2[i].length; k++)
      for (var l: number = 0; l < Enums.length; l++)
      for (var l: number = 0; l < Enums.length; l++)
        if (spaltenTags[i][k] == Enums[l]) enumi.add(Enums[l]);
        if (chks2[i][k] == Enums[l].toString()) enumi.add(Enums[l]);
*/

function subFkt3(Enums: number[], SubFkt3SubFkt2Var: (i3: number, k3?: number) => void | Set<number>, SubFkt3SubFkt1Var: (i3: number, k3?: number) => void | Set<number>, chks2orSpaltenTagsOrTRs: any[][]| HTMLCollectionOf<HTMLTableRowElement>): void {
  /*console.log(chks2orSpaltenTagsOrTRs[0].cells.length)
  console.log(TRs[0].cells.length)
  console.log(spaltenTags.length)*/
  //var counter: number = 0;
  // i sind Spalten oder die checkboxnummer und k sind deren Zeilen
  var enumi2: Array<Set<number>> =new Array();
  var beginI: number = ((chks2orSpaltenTagsOrTRs === spaltenTags) ? 2 : 0);
  var Enum2: ST;
  for (var i: number = beginI; i < chks2orSpaltenTagsOrTRs.length; i++) {
    enumi = new Set();
    for (var k: number = 0; k < ((TRs === chks2orSpaltenTagsOrTRs) ? chks2orSpaltenTagsOrTRs[i].cells.length : chks2orSpaltenTagsOrTRs[i].length); k++) {
      if (chks2orSpaltenTagsOrTRs === TRs) {
        for (var l: number = 0; l < Enums.length; l++) {
          // WICHTIG: eigentlich müsste man aus Typ ST[][]  Typ TS[][][] machen, wo noch mal für jede Spalte einzeln unterschieden wird: Denn dann kann richtiger für die eine Checkbox dieser Spalten bestimmt werden, ob wann diese ausgegraut werden soll, wann dessen Spalten ausgegraut sind. So ist es wie jetzt mehr falsch, aber es ist okay, weil es auch eine eigene Art von Richtigkeit dabei besitzt, weshalb ich das mal so toleriere, denn das hat eine abstraktere Richtigkeit, die okay sein kann!
          Enum2 = spaltenTags[i][k];
          if (Enum2 == Enums[l])
            enumi.add(Enums[l]);
        }
      } else
        for (var l: number = 0; l < Enums.length; l++) {
          // WICHTIG: eigentlich müsste man aus Typ ST[][]  Typ TS[][][] machen, wo noch mal für jede Spalte einzeln unterschieden wird: Denn dann kann richtiger für die eine Checkbox dieser Spalten bestimmt werden, ob wann diese ausgegraut werden soll, wann dessen Spalten ausgegraut sind. So ist es wie jetzt mehr falsch, aber es ist okay, weil es auch eine eigene Art von Richtigkeit dabei besitzt, weshalb ich das mal so toleriere, denn das hat eine abstraktere Richtigkeit, die okay sein kann!
          Enum2 = chks2orSpaltenTagsOrTRs[i][k] as ST;
          if (Enum2 == Enums[l])
            enumi.add(Enums[l]);
        }

        // spaltenTags ist für die Filterung nach Spalten statt Checkboxen
        // spaltenTags und chks2orSpaltenTagsOrTRs haben als Paramter erst Spalten dann Zeilen
        // ich brauche ein map dict, dass abbildet, welche spalten zu welchen checkboxen gehören
        // irgendwie zählt immer die letzte Spalte, zum entscheiden, ob die zugehörige Checkbox disabled oder enabled werden soll. Das sollte so nicht sein.
        // chks2orSpaltenTagsOrTRs kann chks2 sein, was fast wie chks1 ist, nur nach c_ gefiltert.
        // c_ enthält danach die kleinen TaggingNummern für die Checkbox
        // das hier mit c_ muss wohl der Grund sein, warum immer die letzte ausgegraute Spalte für das Ausgrauen der Checkbox sorgt.
        // Ansonsten kann das auch daran liegen, dass der Enum String Vergleich hier oben falsch stattfindet.
        // dafür console out, und für c_: schauen wie die matrix mit c_ gebildet wird, und diese Schöpfung inspizieren und ändern.
    }
    enumi2.push(enumi)
    //console.log(i)
  }
  //console.log(enumi2.length)
  for (var i: number = beginI; i < chks2orSpaltenTagsOrTRs.length; i++) {
    //console.log(i);
    //console.log(enumi);
    if (TRs === chks2orSpaltenTagsOrTRs) {
        TDs = TRs[i].cells as HTMLCollectionOf<HTMLTableCellElement>;
        if (i>4 && i<21) {
            if (!isNaN(TDs[1].innerHTML.trim())) {
                i2 = parseInt(TDs[1].innerHTML.trim())
                //window.alert(TDs[1].innerHTML);
                sPolygon = new StarPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                polyg1 = sPolygon.drawStarPolygon(i, pSize, pSize, 25);
                starPolygons.push(sPolygon);
                //sPolygon.animate();
                gfPolygon = new gleichfPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                polyg2 = gfPolygon.drawPolygon(i, pSize, pSize, 14);
            }
        } else
            if (i<5) {
                if (!isNaN(TDs[1].innerHTML.trim())) {
                    //window.alert(TDs[1].innerHTML);
                    i2 = parseInt(TDs[1].innerHTML.trim())
                    gfPolygon = new gleichfPolygon(pSize*2, alleMonde.includes(i2) ? 'white' : 'black');
                    polyg2= gfPolygon.drawPolygon(i, pSize, pSize, 15);
                }
            }
      if (i == 1)
        SubFkt3SubFkt2Var(i);
    }
    //console.log(i)
    //console.log(enumi2.length)
    enumi = enumi2[i-beginI];
    if ((enumi.has(ST.sternPolygon) || enumi.has(ST.gleichfoermigesPolygon) || enumi.has(ST.gebrRat)) &&
      (enumi.has(ST.galaxie) || enumi.has(ST.universum) || enumi.has(ST.keinParaOdMetaP)) &&
      enumi.size != 0) {
      SubFkt3SubFkt2Var(i);
    } else {
      SubFkt3SubFkt1Var(i);
    }
  }
}

var SubFkt3SubFkt2Ptr2: (i: number, k?: number) => void | Set<number> = function SubFkt3SubFkt2(i: number,k: number=0): void {
    for (var k: number = 0; k < spalten4spaltenTags[i].length; k++) {
        spalten4spaltenTags[i][k].style.fontSize = "80%";
        spalten4spaltenTags[i][k].style.opacity = "0.4";
    }
}

var SubFkt3SubFkt1Ptr2: (i: number, k?: number) => void | Set<number> = function SubFkt3SubFkt1(i: number,k: number=0): void {
    for (var k: number = 0; k < spalten4spaltenTags[i].length; k++) {
        spalten4spaltenTags[i][k].style.fontSize = "100%";
        spalten4spaltenTags[i][k].style.opacity = "1.0";
    }
}



var SubFkt3SubFkt1Ptr: (i: number, k?: number) => void | Set<number> = function SubFkt3SubFkt2(i: number,k: number=0): void {
  chks1[i].disabled = false;
  chks1[i].style.fontSize = tdStyleFontSize;
  chks1[i].style.color = "";
  chks1[i].style.whiteSpace = tdStyleWhiteSpace;
}

var SubFkt3SubFkt2Ptr: (i: number, k?: number) => void | Set<number> = function nubFkt3SubFkt1(i: number,k: number=0): void {
  chks1[i].disabled = true;
  chks1[i].style.fontSize = tdStyleFontSizeKl;
  chks1[i].style.color = tdStyleColorKl;
  chks1[i].style.whiteSpace = tdStyleWhiteSpace;
}

function makeSpacesOutOf_(text: string): string {
  //if (text.length == 10) if (text == "Wichtigste") return "<b>Wichtigste</b>";
    //
  if (text.includes("wichtig") || text.includes("Wichtig") || text.includes("Grundstrukturen") || text.includes("(15)")) text = "<b>"+text+"</b>";
  /*if (text.length == 25)
    if (text == "Wichtigstes_zum_verstehen")
      return "<b>Wichtigstes zum verstehen</b>";
  if (text.length == 36)
    if (text == "Wichtigstes_zum_gedanklich_einordnen")
      return "<b>Wichtigstes zum gedanklich einordnen</b>";
  /*
  if (text.length == 8) if (text == "zaehlung") return "Zählung";
  if (text.length == 12) if (text == "nummerierung") return "Nummerierung";
  if (text.length == 11)
    if (text == "kombination") return "Kombinationen (Galaxie)";
  if (text.length == 18)
    if (text == "gebrochenuniversum") return "gebrochen-rational Universum n/m";
  if (text.length == 16)
    if (text == "gebrochengalaxie") return "gebrochen-rational Galaxie n/m";
  */
  var forNewString: string[] = [];
  for (var i = 0; i < text.length; i++)
    if (text[i] == "_") forNewString.push(" ");
    else forNewString.push(text[i]);
  return forNewString.join("");
}

function copyClassNameToOrderedGrunstruk(
  mapMapMap: Map<string, string | Map<string , string | Map<any, any>>> ,
  mapMapMapTags: Map<number, string>,
  p1keys: string[],
  p2keys: Array<string | Map<any, any>>,
  grunSi: number,
  grunp2Keys: Array<string | Map<any, any>>
) {
  //checkboxesOrdnung = document.getElementsByClassName("ordGru");
  //checkboxesChaos = document.getElementsByClassName("chks");

  //var p1keysB = Object.keys(mapMapMap);
  //var p2keysB = Object.keys(mapMapMap["✗Grundstrukturen"]);
  //numbers = Array.from(mapMapMap["✗Grundstrukturen"][p2keys[0]]);
  //grundstrukThings = Array.from(mapMapMap["✗Grundstrukturen"]);
  //window.alert(String(numbers.join(",")));
  //window.alert(String(grundstrukThings[0].join(",")));
  // (p1keys[i] === "✗Grundstrukturen"
  // var p1keys = Object.keys(mapMapMap);
  //var p1keyGrund = Object.keys(mapMapMap["✗Grundstrukturen"]);
  //var p2keys = Object.keys(mapMapMap[p1);
  // var thingsB = Array.from(mapMapMapTags[p2keyGrund][p2keys[k]]).join(",");
  //var thingsB = Array.from(mapMapMapTags[p1keyGrund]);
  var TagIdGrustruk: any = document.getElementById("✗Grundstrukturen");
  var chaotische: Array<any> = [];
  var ordentliche: Array<any> = [];
  var ordentliche2: Array<any> = [];
  for (var i: number = 0; i < grunp2Keys.length; i++) {
    var nummern: string = Array.from(mapMapMapTags[p1keys[grunSi]][grunp2Keys[i]]).join(
      ","
    );
    var ordentlich: any = document.getElementById("ordGru" + grunp2Keys[i]);
    var ordentlich2: any = document.getElementById("ordGruB" + grunp2Keys[i]);
    var chaotisch: HTMLCollectionOf<HTMLInputElement>  = TagIdGrustruk.getElementsByClassName("chks c_" + nummern);
    if (typeof chaotisch !== "undefined" && chaotisch !== null)
      chaotische.push(chaotisch);
    if (typeof ordentlich !== "undefined" && ordentlich !== null)
      ordentliche.push(ordentlich);
    if (typeof ordentlich2 !== "undefined" && ordentlich2 !== null)
      ordentliche2.push(ordentlich2);
    /*
    if (ordentlich !== null) {
      blax = ordentlich.parentElement.getElementsByClassName("OrdGru2");
      if (blax !== null && typeof blax !== "undefined") {
        try {
          //blay = blax.getElementsByTagName("label");

          window.alert(String(blax.innerHTML));
        } catch (error) {}
        //window.alert(String(blay.length));
      }
    }*/
    //bla3[zahl 0 bis n] value sind richtige Namen der checkboxen

    //'" ><input type="checkbox" class="chks c_' +
    //Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(",") +

    /*if (
      ordentlich !== null &&
      //typeof ordentlich.value !== "undefined" //&&
      //ordentlich.getElementsByTagName("label").length > 0
      ordentlich.innerHTML != ""
    )
      window.alert(String(ordentlich.innerHTML));
*/
    //  try {
    /*for (var k = 0; k < bla3.length; k++) {
      if (typeof bla3[k].value !== "undefined") {
        window.alert(
          //String(mapMapMapTags[p1keys[grunSi]][grunp2Keys[i]].join(","))
          // String(p1keys[grunSi]) ===== "Grund...."
          // String(grunp2Keys[i]) === diese dinge da drin
          String(bla3[k].value)
        );
      }
    }*/
    //} catch (error) {}
    //
  }
  //window.alert(String(chaotische.length));
  //window.alert(String(ordentliche.length));
  for (var i: number = 0; i < chaotische.length; i++) {
    chaotisch = chaotische[i];
    for (var m: number = 0; m < chaotisch.length; m++) {
      for (var k: number = 0; k < ordentliche.length; k++) {
        if (
          typeof ordentliche[k].value !== "undefined" &&
          chaotisch != null &&
          typeof chaotisch[m].value !== "undefined" &&
          chaotisch[m].className.substring(0, 4) === "chks" &&
          ordentliche[k].value === chaotisch[m].value
        ) {
          ordentliche[k].className =
            "chks " + chaotisch[m].className.substring(4);
          ordentliche2[k].className =
            "chks " + chaotisch[m].className.substring(4);
          //window.alert(String(ordentliche[k].className));
        }
      }
      // Der Klassen-Inhalt setzt sich zusammen aus:
      //       '" ><input type="checkbox" class="chks c_' +
      //       Array.from(mapMapMapTags[p1keys[i]][p2keys[k]]).join(",") +
    }
  }
}
function grundSDivToggleBeachte(para: string = "", dasTag: boolean = false) {
  var checkboxesOrdnung: HTMLCollectionOf<any> = document.getElementsByClassName("ordGru");
  var checkboxesChaos: HTMLCollectionOf<any> = document.getElementsByClassName("chks");
  if (para !== "") {
    if (!dasTag) {
      for (var i: number = 0; i < checkboxesOrdnung.length; i++) {
        for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
          if (typeof checkboxesChaos[i] !== "undefined" && typeof checkboxesChaos[i].value !== "undefined" && k != i) {
            //window.alert(String(checkboxesChaos[i].value));
            if (
              checkboxesOrdnung[k].value === checkboxesOrdnung[i].value
              // checkboxesOrdnung[k].checked != checkboxesOrdnung[i].checked
            ) {
              if (checkboxesOrdnung[k].value === para) {
                checkboxesOrdnung[k].checked = false;
                checkboxesOrdnung[i].checked = false;
              }
            }
          }
        }
      }
      for (var i: number = 0; i < checkboxesChaos.length; i++) {
        for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
          if (typeof checkboxesChaos[i] !== "undefined" && typeof checkboxesChaos[i].value !== "undefined") {
            //window.alert(String(checkboxesChaos[i].value));
            if (
              checkboxesOrdnung[k].value === checkboxesChaos[i].value
              //checkboxesOrdnung[k].checked != checkboxesChaos[i].checked
            ) {
              if (checkboxesOrdnung[k].value === para) {
                checkboxesOrdnung[k].checked = false;
                checkboxesChaos[i].checked = false;
              }
            }
          }
        }
      }
    } else {
      if (dasTag) {
        for (var i: number = 0; i < checkboxesOrdnung.length; i++) {
          for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
            if (typeof checkboxesChaos[i] !== "undefined" && typeof checkboxesChaos[i].value !== "undefined" && k != i) {
              //window.alert(String(checkboxesChaos[i].value));
              if (
                checkboxesOrdnung[k].value === checkboxesOrdnung[i].value
                //checkboxesOrdnung[k].checked != checkboxesOrdnung[i].checked
              ) {
                if (checkboxesOrdnung[k].value === para) {
                  checkboxesOrdnung[k].checked = true;
                  checkboxesOrdnung[i].checked = true;
                }
              }
            }
          }
        }
        for (var i: number = 0; i < checkboxesChaos.length; i++) {
          for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
            if (typeof checkboxesChaos[i] !== "undefined" && typeof checkboxesChaos[i].value !== "undefined") {
              //window.alert(String(checkboxesChaos[i].value));
              if (
                checkboxesOrdnung[k].value === checkboxesChaos[i].value
                // checkboxesOrdnung[k].checked != checkboxesChaos[i].checked
              ) {
                if (checkboxesOrdnung[k].value === para) {
                  checkboxesOrdnung[k].checked = true;
                  checkboxesChaos[i].checked = true;
                }
              }
            }
          }
        }
      }
    }
  } else {
    for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
      for (var k2: number = 0; k2 < checkboxesOrdnung.length; k2++) {
        if (k != k2) {
          //window.alert(String(checkboxesChaos[i].value));
          if (
            checkboxesOrdnung[k2].value === checkboxesOrdnung[k].value &&
            checkboxesOrdnung[k2].checked != checkboxesOrdnung[k].checked
          ) {
            if (checkboxesOrdnung[k2].checked == false)
              checkboxesOrdnung[k2].checked = true;
          }
        }
      }
    }
    for (var i: number = 0; i < checkboxesChaos.length; i++) {
      for (var k: number = 0; k < checkboxesOrdnung.length; k++) {
        if (typeof checkboxesChaos[i] !== "undefined" && typeof checkboxesChaos[i].value !== "undefined" && k != i) {
          //window.alert(String(checkboxesChaos[i].value));
          if (
            checkboxesOrdnung[k].value === checkboxesChaos[i].value &&
            checkboxesOrdnung[k].checked != checkboxesChaos[i].checked
          ) {
            if (checkboxesOrdnung[k].checked == false)
              checkboxesOrdnung[k].checked = true;
            if (checkboxesChaos[i].checked == false)
              checkboxesChaos[i].checked = true;
          }
        }
      }
    }
  }
}
function grundSDivToggle(id_: number) {
  //checkboxesChaos = document.getElementsByTagName("input");
  if (id_ == 1) {
    document.getElementById("grundRadioChaos").checked = false;
    document.getElementById("grundRadioOrdnung").checked = true;
    document.getElementById("grundSDiv0").style.display = "none";
    document.getElementById("grundSDiv1").style.display = "inline";
  } else {
    document.getElementById("grundRadioChaos").checked = true;
    document.getElementById("grundRadioOrdnung").checked = false;
    document.getElementById("grundSDiv0").style.display = "inline";
    document.getElementById("grundSDiv1").style.display = "none";
    //checkboxes = document.getElementsByClassName("ordentlicheGrundStrukChk");
    //for (var checkbox in checkboxes) {
    //  checkbox.checked = false;
    //}
  }
  grundSDivToggleBeachte("", false);
  //window.alert(String(checkboxesOrdnung.length));
  //
}

function toggleP2(dasTag: HTMLInputElement, spaltenNummern1: Array<Map<any,any>> | string, para1u2: string) {

  //console.log("para1u2: ", para1u2)
  var pa1u2: string[] = para1u2.split(",");
  var spaltenNummern: string[];
  try {
    /*window.alert(String();
    window.alert(String(pa1u2[1]));
    window.alert(String(Array.from(mapMapMap[pa1u2[0]][pa1u2[1]])));*/
    spaltenNummern = Array.from(mapMapMap[pa1u2[0]][pa1u2[1]]);
    //window.alert(String(spaltenNummern));
  } catch (error) {
    spaltenNummern = spaltenNummern1.split(",");
  }
  var existingParameterNamesArrayIndex: Set<number> = MatrixHasCouple(
    para1u2,
    selectedSpaltenMany2
  );
  if (existingParameterNamesArrayIndex.size > 0) {
    var existingParameterNamesKeys: any[] = Array.from(existingParameterNamesArrayIndex);
    for (var i: number = 0; i < existingParameterNamesKeys.length; i++) {
      for (
        var k: number = 0;
        k < selectedSpaltenMany2[existingParameterNamesKeys[i]].length;
        k++
      ) {
        if (selectedSpaltenMany2[existingParameterNamesKeys[i]][k] == para1u2) {
          selectedSpaltenMany2[existingParameterNamesKeys[i]].splice(k, 1);
        } else {
        }
      }
    }
    toggleForNums(spaltenNummern);
  } else {
    for (var i: number = 0; i < spaltenNummern.length; i++)
      if (typeof selectedSpaltenMany2[spaltenNummern[i]] !== "undefined")
        selectedSpaltenMany2[spaltenNummern[i]].push(para1u2);
      else selectedSpaltenMany2[spaltenNummern[i]] = [para1u2];
    toggleForNums(spaltenNummern);
  }
  grundSDivToggleBeachte(pa1u2[1], dasTag.checked);
}

function MatrixHasCouple(couple: string, SpaltenNumberToParameters: Map<number,string>): Set<number> {
  var existing: Set<any> = new Set();
  for (var key in SpaltenNumberToParameters) {
    for (var i: number = 0; i < SpaltenNumberToParameters[key].length; i++) {
      for (var k: number = 0; k < SpaltenNumberToParameters[key].length; k++) {
        if (SpaltenNumberToParameters[key][k] != couple) {
        } else {
          existing.add(key);
        }
      }
    }
  }
  return existing;
}

function toggleForNums(colNums: string[]) {
  for (var n: number = 0; n < colNums.length; n++) {
    /*if (typeof(selectedSpaltenMany2[colNums]) === 'undefined')
			toggleSpalten(colNums[n]);
		else {
			toggleSpalten(colNums[n]);
		}*/
    toggleSpalten(parseInt(colNums[n]));
  }
  //window.alert("colNums 0:"+colNums[0])
  refresh();
}

function refresh() {
  sortedKeysOfHeadingNumbersByVisibility();
  //console.log("refresh");
  setAllListsInHeadings();
  updateVisibleHeadingsNumbersAndItsKeysList();
}

function updateVisibleHeadingsNumbersAndItsKeysList() {
  var keys: Array<number | string> = Object.keys(visibleHeadingsSelectUnsorted);
  for (var i: number = 0; i < keys.length; i++) {
    visibleHeadingsNumbers[keys[i]] =
      visibleHeadingsSelectUnsorted[keys[i]].value;
  }
  var keys2: string[] = Object.keys(visibleHeadingsNumbers);
  //window.alert("vis num"+ keys2.length)
  //window.alert("vis num 0: "+ visibleHeadingsNumbers[keys2[0]])
}

function toggleName(p2: HTMLElement) {
  if (p2.style.display != "none") p2.style.display = "none";
  else if (p2.getElementsByTagName("input").length > 0) {
    p2.style.display = "block";
    p2.style.fontSize = "100%";
    //animateAllPolygons();
  }
}

function toggleP1(p1: string) {

  //console.log("p1:", p1)
  var p2: HTMLElement | null  = document.getElementById(p1);
  if (p2 != null && typeof p2.style != "undefined") {
    var num1: RegExpMatchArray | null  = p2.className.match(/r_(\d+)/);
    var num: number | RegExpMatchArray | null;
    //console.log("num A:", num1);
    if (num1 != null && num1.length > 1) {
        num = parseInt(num1[1]);
    } else num=num1;
    //console.log("num B:", num);
    //console.log("typ", typeof num);
    if (
      (typeof selectedSpaltenMany1[num] === "undefined") ===
      (p2.style.display != "none")
    ) {
      selectedSpaltenMany1[num] = p2;
      toggleName(p2);
    } else {
      toggleName(p2);
      delete selectedSpaltenMany1[num];
    }

  } else window.alert(p2.innerHTML + " ! ");
}

function toggleSpalten(colNumber: number) {
  var ZeileIhreZellen: HTMLCollectionOf<HTMLElement> | HTMLElement[] = document.getElementsByClassName("r_" + colNumber);
  if (typeof selectedSpaltenMany2[colNumber] === "undefined") {
    var away: boolean = true;
  } else away = selectedSpaltenMany2[colNumber].length == 0;
  //window.alert("Stelle "+colNumber+"hat Länge "+selectedSpaltenMany2[colNumber].length);
  if (typeof ZeileIhreZellen[0].style != "undefined") {
    if (ZeileIhreZellen[0].style.display == "none")
      changeHeadline(ZeileIhreZellen[0], true);
    else if (away) changeHeadline(ZeileIhreZellen[0], false);

    if (ZeileIhreZellen[0].getElementsByTagName("option").length == 0)
      var spalteEinzelnDeaktiviertWorden: boolean = false;
    else if (ZeileIhreZellen[0].getElementsByTagName("option")[0].selected)
      spalteEinzelnDeaktiviertWorden = true;
    else spalteEinzelnDeaktiviertWorden = false;

    for (var i: number = 0; i < ZeileIhreZellen.length; i++) {
      if (
        ZeileIhreZellen[i].style.display == "none" &&
        !spalteEinzelnDeaktiviertWorden
      ) {
        ZeileIhreZellen[i].style.display = "table-cell";
        ZeileIhreZellen[i].style.fontSize = "100%";
        //animateAllPolygons();
      } else if (away || spalteEinzelnDeaktiviertWorden) {
        ZeileIhreZellen[i].style.display = "none";
      }
    }
    if (spalteEinzelnDeaktiviertWorden) {
      //window.alert('B '+ZeileIhreZellen[0].className.match(/r_(\d+)/g)[0]);
      //window.alert('B '+ZeileIhreZellen[0].className.match(/r_(\d+)/g)[0].substring(2));
      try {
        delete visibleHeadingsSelectUnsorted[
            parseInt(ZeileIhreZellen[0].className.match(/r_(\d+)/g)[0].substring(2))
        ];
      } catch {}
      // sie wieder zu aktivieren, auf 1 statt 0 setzen (wobei hier die richtige zahl eigentlich besser wäre)
      // auf 1 setzen ist aber okay, weil die durch refresh usw. sowieso wieder umgesetzt wird
      ZeileIhreZellen[0].getElementsByTagName("option")[1].selected = true;
    }
  } else window.alert(ZeileIhreZellen[0].innerHTML + " ! " + colNumber);
}

var tableHeadline: HTMLTableCellElement;
var visibleHeadingsSelectUnsorted: Map<number, HTMLElement> = new Map();
//var visibleHeadingsNumbers: Map<number, Map<number, HTMLElement>> = new Map();
var visibleHeadingsNumbers: Map<number, string[]> = new Map();

function changeHeadline(oneColHeading: HTMLTableCellElement, addTrueRemoveFalse: boolean) {
  var sel: HTMLSelectElement = oneColHeading.getElementsByTagName("select")[0];
  var num1: RegExpMatchArray  = oneColHeading.className.match(/r_(\d+)/g);
  var num: number;
  if (!!num1 && num1.length > 0) num = parseInt(num1[0].substring(2));
  else num = 0;
  //window.alert(num);

  if (addTrueRemoveFalse) visibleHeadingsSelectUnsorted[num] = sel;
  else if (num in visibleHeadingsSelectUnsorted)
    delete visibleHeadingsSelectUnsorted[num];
  //window.alert(Object.keys(visibleHeadingsSelectUnsorted).length);
  //
}

function makeSpalteUnsichtbar(
  spalteToUnsichtbar: HTMLCollectionOf<HTMLTableCellElement>,
  momentaneSpalte_als_r_: number,
  hiddenTrueVisibleFalse: boolean
) {
  //spalteToUnsichtbar = document.getElementsByClassName("r_"+momentaneSpalte_als_r_);
  var len: number = spalteToUnsichtbar.length;
  if (hiddenTrueVisibleFalse) {
    for (var i: number = 0; i < len; i++) spalteToUnsichtbar[i].style.display = "none";
    delete visibleHeadingsSelectUnsorted[momentaneSpalte_als_r_];
  } /*else {
        for (var i=0; i<len; i++)
            spalteToUnsichtbar[i].style.display = 'table-cell'
        visibleHeadingsSelectUnsorted['r_'+momentaneSpalte_als_r_]=spalteToUnsichtbar;
    }*/
}

var erstesMal: boolean = true;

function headingSelected(gewaehlteSpalte_plusgleich1a: HTMLSelectElement, momentaneSpalte_als_r_1: string) {
  var momentaneSpalte_als_r_ : number = parseInt(momentaneSpalte_als_r_1)
  var gewaehlteSpalte_plusgleich1: string | number = gewaehlteSpalte_plusgleich1a.value;
  //for (var i=0; i<optionsS.length; i++) {
  var zwei: string[] = gewaehlteSpalte_plusgleich1.split(",");
  gewaehlteSpalte_plusgleich1 = zwei[0];
  var gewaehlteSpalte_als_r_: string = zwei[1];
  var spalte2ToChange: HTMLCollectionOf<HTMLTableCellElement | Element> = document.getElementsByClassName(
    "r_" + momentaneSpalte_als_r_
  );
  if (gewaehlteSpalte_plusgleich1 == "-") {
    makeSpalteUnsichtbar(spalte2ToChange, momentaneSpalte_als_r_, true);
    refresh();
    return;
  }
  if (erstesMal) {
    //window.alert("Das Dauert! Geduld mitbringen! Alles friert kurz ein!");
    erstesMal = false;
  }
  //var momentaneSpalte_plusgleich1: Map<number, HTMLElement> = visibleHeadingsNumbers[momentaneSpalte_als_r_]; // dieses mal als +=1 angabe statt als r_
  var momentaneSpalte_plusgleich1: string = visibleHeadingsNumbers[momentaneSpalte_als_r_]; // dieses mal als +=1 angabe statt als r_
  var zwei: string[] = momentaneSpalte_plusgleich1.split(",");
  var momentaneSpalte_plusgleich1: string = zwei[0];
  var spalte1ToChange: HTMLCollectionOf<HTMLTableCellElement | Element> = document.getElementsByClassName(
    "r_" + gewaehlteSpalte_als_r_
  );
  var seli: HTMLCollectionOf<HTMLOptionElement> = spalte1ToChange[0]
    .getElementsByTagName("select")[0]
    .getElementsByTagName("option");
  var selival: number = parseInt(selectionsBefore[momentaneSpalte_plusgleich1]) + 1;
  gewaehlteSpalte_plusgleich1 = parseInt(selival) - 2; // 1 bis +=1
  seli[selival].selected = true;
  var tabellenKopf = document.getElementsByClassName("z_0");
  var aa = 0;
  var bb = 0;
  for (var z = 0; z < tabellenKopf.length; z++) {
    if (tabellenKopf[z] === spalte2ToChange[0]) aa = z;
    if (tabellenKopf[z] === spalte1ToChange[0]) bb = z;
  }

  var merke;
  if (aa > bb)
    for (var i = 0; i < spalte1ToChange.length; i++) {
      merke = spalte2ToChange[i].outerHTML;
      spalte2ToChange[i].outerHTML = spalte1ToChange[i].outerHTML;
      spalte1ToChange[i].outerHTML = merke;
    }
  else
    for (var i = 0; i < spalte1ToChange.length; i++) {
      merke = spalte1ToChange[i].outerHTML;
      spalte1ToChange[i].outerHTML = spalte2ToChange[i].outerHTML;
      spalte2ToChange[i].outerHTML = merke;
    }

  visibleHeadingsSelectUnsorted[gewaehlteSpalte_als_r_] =
    spalte1ToChange[0].getElementsByTagName("select")[0];
  visibleHeadingsSelectUnsorted[momentaneSpalte_als_r_] =
    spalte2ToChange[0].getElementsByTagName("select")[0];
  refresh();
}

var selectionsBefore: Map<number, number> = new Map();
var optionsS: string[][]  = [];
var sichtbareSpaltenNummern: string[];

function sortedKeysOfHeadingNumbersByVisibility() {
  var tableHeadline: HTMLCollectionOf<HTMLTableCellElement> = document.getElementById("bigtable").rows[0].cells;
  sichtbareSpaltenNummern = [];
  for (var i: number = 0; i < tableHeadline.length; i++) {
    if (tableHeadline[i].style.display == "table-cell") {
      sichtbareSpaltenNummern.push(
        tableHeadline[i].className.match(/r_(\d+)/g)[0].substring(2)
      );
    }
  }
  //reihenfolgenstring = sichtbareSpaltenNummern.join(", ");
  //window.alert('sichtb spalten r_ nummern: '+reihenfolgenstring);
}

function setAllListsInHeadings() {
  var options: string[];
  var optionsS: string[][]  = [];
  var keys: Array<number | string> = Object.keys(visibleHeadingsSelectUnsorted);
  var len: number = keys.length;
  for (var k: number = 0; k < len; k++) {
    options = ["<option value='-,null'>-</option>"];
    for (var i: number = 0; i < len; i++)
      if (i != k)
        options.push(
          "<option value='" +
            i +
            "," +
            sichtbareSpaltenNummern[i] +
            "'>" +
            (i + 1) +
            "</option>"
        );
      else {
        options.push(
          "<option selected value='" +
            i +
            "," +
            sichtbareSpaltenNummern[i] +
            "'>" +
            (i + 1) +
            "</option>"
        );
        var selection: number = i;
      }
    selectionsBefore[k] = k;
    optionsS.push(options);
  }
  if (len != sichtbareSpaltenNummern.length) {
    window.alert(
      "beides sichtbares und beide Längen nicht gleich: td spalten zellen anzahl als dict mir _r keys und die _r Nummerierung derer als array, sichtbareSpaltenNummern ist " +
        sichtbareSpaltenNummern.length +
        " und visibleHeadingsSelectUnsorted ist " +
        len
    );
  }
  for (var i: number = 0; i < sichtbareSpaltenNummern.length; i++) {
    visibleHeadingsSelectUnsorted[sichtbareSpaltenNummern[i]].innerHTML =
      optionsS[i].join("");
  }
}

function toggleChkSpalten() {
  chk_spalten: HTMLDivElement = document.getElementById("chk_spalten");
  inputZeilen: HTMLDivElement = document.getElementById("inputZeilen");
  spaltenWahl: HTMLInputElement = document.getElementById("spaltenWahl");
  zeilenWahl: HTMLInputElement = document.getElementById("zeilenWahl");

  if (inputZeilen.style.display == "none" && zeilenWahl.checked)
    inputZeilen.style.display = "initial";
  else if (!zeilenWahl.checked) inputZeilen.style.display = "none";

  if (chk_spalten.style.display == "none" && spaltenWahl.checked)
    chk_spalten.style.display = "initial";
  else if (!spaltenWahl.checked) chk_spalten.style.display = "none";
}

/*
function potenzenAngabenToContainer() {
  text = document.getElementById("potenzenErlaubtText").value;
  var zeilenAngaben = new Set();
  text = text.split(",");
  for (var i = 0; i < text.length; i++) {
    number = parseInt(text[i]);
    if (number != "NaN") zeilenAngaben.add(number);
  }
  return zeilenAngaben;
}
*/
/*function isZeilenAngabe_betweenKommas(g) {
  const x = (g.match(/[0-9]+-[0-9]+/g) || []).length;
  const y = (g.match(/[0-9]+-[0-9]+-[0-9]+/g) || []).length;
  return (
    /^([0-9-]+[\+0-9,-]*)$/.test(g) &&
    !["-", "+"].includes(g.slice(-1)) &&
    ((x < 2 && y == 0) || (/^\-?[0-9]+[\+0-9,]*$/.test(g) && x == 0)) &&
    !/--|\+\+|\+\-|\-\+|,\+|\+,|-,/.test(g)
  );
}*/
function isZeilenAngabe_betweenKommas(g: string): boolean {
  const pattern: RegExp = /^(v?-?\d+)(-\d+)?((\+)(\d+))*$/;
  return !!g.match(pattern);
  /*const x = (g.match(/[0-9]+-[0-9]+/g) || []).length;
  const y = (g.match(/[0-9]+-[0-9]+-[0-9]+/g) || []).length;
  return (
    /^([0-9-]+[\+0-9,-]*)$/.test(g) &&
    !["-", "+"].includes(g.slice(-1)) &&
    ((x < 2 && y == 0) || (/^\-?[0-9]+[\+0-9,]*$/.test(g) && x == 0)) &&
    !/--|\+\+|\+\-|\-\+|,\+|\+,|-,/.test(g)
  );*/
}
function isZeilenAngabe(text: string): boolean {
  var text: string;
  if (text.length > 0 && text[0] === "v") {
    text = text.substring(1);
  }
  var a: boolean[] = [];
  var splittedText: string[] = text.split(",");
  for (var i = 0; i < splittedText.length; i++) {
    a.push(isZeilenAngabe_betweenKommas(splittedText[i]));
  }

  return a.every(function (x) {
    return x;
  });
}
/*
function isZeilenAngabe(g) {
  const x = (g.match(/[0-9]+\-[0-9]+/g) || []).length;
  const y = (g.match(/[0-9]+\-[0-9]+\-[0-9]+/g) || []).length;
  return (
    /^(v?[0-9-]+[\+0-9,-]*)$/.test(g) &&
    !["-", "+"].includes(g.slice(-1)) &&
    ((x < 2 && y == 0) || (/^v?\-?[0-9]+[\+0-9,]*$/.test(g) && x == 0)) &&
    !/--|\+\+|\+\-|\-\+|,\+|\+,|-,/.test(g)
  );
}
*/
/*
function isZeilenAngabe(g) {
  let pattern = new RegExp("^v?[0-9-]+[\\+0-9,-]*$");
  return (
    pattern.test(g) &&
    !["-", "+"].includes(g.slice(-1)) &&
    !g.includes("--") &&
    !g.includes("++") &&
    !g.includes("+-") &&
    !g.includes("-+") &&
    !g.includes(",+") &&
    !g.includes("+,") &&
    !g.includes("-,")
  );
}
*/
function BereichToNumbers2(MehrereBereiche: string, vielfache: boolean = false, maxZahl: number = 1028): Set<number> {
  var MehrereBereiche: string = MehrereBereiche.split(",")
    .filter((s) => s.trim())
    .join(",");
  //console.log(MehrereBereiche)
  const Bereiche: string[] = MehrereBereiche.split(",");
  if (!isZeilenAngabe(MehrereBereiche)) {
    return new Set();
  }

  if (!vielfache && maxZahl === 0) {
    maxZahl = Infinity;
  }

  var dazu: Set<number> = new Set();
  var hinfort: Set<number> = new Set();
  var EinBereich: string;
  var EinBereich2: string;
  var vielfache2: boolean = true;
  var vielfache: boolean;

  for (const EinBereich of Bereiche) {
    if (EinBereich.length > 0 && EinBereich[0] === "v") {
      EinBereich2 = EinBereich.slice(1);
      vielfache2 = true;
    } else {
      EinBereich2 = EinBereich;
      vielfache2 = false;
    }
    //window.alert(EinBereich);
    BereichToNumbers2_EinBereich(
      EinBereich2,
      dazu,
      hinfort,
      (vielfache || vielfache2) && maxZahl == Infinity ? 1028 : maxZahl,
      vielfache || vielfache2,
    );
  }

  //console.log(new Set<number>([...dazu].filter((x) => !hinfort.has(x))));
  return new Set<number>([...dazu].filter((x) => !hinfort.has(x)));
}

function BereichToNumbers2_EinBereich(
  EinBereich: string,
  dazu: Set<number>,
  hinfort: Set<number>,
  maxZahl: number,
  vielfache: boolean,
) {
  var menge: Set<number>  = null;
  if (EinBereich.length > 1 && EinBereich[0] === "-") {
    EinBereich = EinBereich.substring(1);
    menge = hinfort;
  } else if (EinBereich.length > 0 && EinBereich[0] !== "-") {
    menge = dazu;
  } else {
    menge = null;
  }

  const around: number[] = [];
  if (menge !== null) {
    const BereichTuple2: string[] = EinBereich.split("+");
    if (EinBereich.match(/^\d+$/)) {
      EinBereich = EinBereich + "-" + EinBereich;
    } else if (BereichTuple2.length > 0 && BereichTuple2[0].match(/^\d+$/)) {
      EinBereich = BereichTuple2[0] + "-" + BereichTuple2[0];
      if (BereichTuple2.length > 1) {
        EinBereich += "+" + BereichTuple2.slice(1).join("+");
      }
    }
    const BereichCouple: string[] = EinBereich.split("-");
    BereichToNumbers2_EinBereich_Menge(
      BereichCouple,
      around,
      maxZahl,
      menge,
      vielfache
    );
  }
}

function BereichToNumbers2_EinBereich_Menge(
  BereichCouple: string[],
  around: number[],
  maxZahl: number,
  menge: Set<number>,
  vielfache: boolean
) {
  var richtig: boolean;
  if (
    BereichCouple.length == 2 &&
    /^\d+$/.test(BereichCouple[0]) &&
    BereichCouple[0] != "0"
  ) {
    let BereichPlusTuples = BereichCouple[1].split("+");
    if (BereichPlusTuples.length < 2) {
      around = [0];
    } else {
      let richtig = true;
      let numList = [];
      for (let i = 0; i < BereichPlusTuples.length; i++) {
        if (/^\d+$/.test(BereichPlusTuples[i])) {
          numList.push(parseInt(BereichPlusTuples[i]));
        } else {
          richtig = false;
        }
      }
      if (richtig && numList.length > 0) {
        around = numList.slice(1);
        BereichCouple[1] = numList[0].toString();
      }
    }
    if (vielfache) {
      BereichToNumbers2_EinBereich_Menge_vielfache(
        BereichCouple,
        around,
        maxZahl,
        menge
      );
    } else {
      BereichToNumbers2_EinBereich_Menge_nichtVielfache(
        BereichCouple,
        around,
        maxZahl,
        menge
      );
    }
  }
}

function BereichToNumbers2_EinBereich_Menge_vielfache(
  BereichCouple: string[],
  around: number[],
  maxZahl: number,
  menge: Set<number>
) {
  let i: number = 0;
  var aroundSet: Set<number> = new Set(around);
  aroundSet.delete(0);
  //window.alert(Array.from(around).join(","));
  if (around.length === 0 || aroundSet.size == 0) {
    while (around.every((a) => parseInt(BereichCouple[0]) * i < maxZahl - a)) {
      i += 1;
      for (
        let number = parseInt(BereichCouple[0]);
        number <= parseInt(BereichCouple[1]);
        number++
      ) {
        for (const a of around) {
          const c = number * i;
          if (c <= maxZahl) {
            menge.add(c);
          }
        }
      }
    }
  } else {
    while (around.every((a) => parseInt(BereichCouple[0]) * i < maxZahl - a)) {
      i += 1;
      for (
        let number = parseInt(BereichCouple[0]);
        number <= parseInt(BereichCouple[1]);
        number++
      ) {
        for (const a of around) {
          const c = number * i + a;
          if (c <= maxZahl) {
            menge.add(c);
          }
          const d = number * i - a;
          if (d > 0 && d < maxZahl) {
            menge.add(d);
          }
        }
      }
    }
  }
}
function BereichToNumbers2_EinBereich_Menge_nichtVielfache(
  BereichCouple: string[],
  around: number[],
  maxZahl: number,
  menge: Set<number>
) {
  for (
    let number = parseInt(BereichCouple[0]);
    number <= parseInt(BereichCouple[1]);
    number++
  ) {
    for (let a of around) {
      let c = number + a;
      if (c < maxZahl) {
        menge.add(c);
      }
      let d = number - a;
      if (d > 0 && d < maxZahl) {
        menge.add(d);
      }
    }
  }
}
function zeilenAngabenToMengeDirekt(welches: number = 0, v: boolean = false) {
  let text: string;
  switch (welches) {
    case 1:
      text = document.getElementById("zeilenErlaubtText").value;
      break;
    case 2:
      text = document.getElementById("zaehlungErlaubtText").value;
      break;
    case 3:
      text = document.getElementById("primVielfache").value;
      break;
    case 4:
      text = document.getElementById("primZahlKreuzRadius").value;
      break;
    case 5:
      text = document.getElementById("VielfacheErlaubtText").value;
      break;
    case 6:
      text = document.getElementById("potenzenErlaubtText").value;
      break;
    default:
      text = "Ungültige Auswahl";
      break;
  }
  erlaubteZeilen  = BereichToNumbers2(text, welches == 5 || v ? true : false);
  //window.alert(Array.from(erlaubteZeilen).join(" "));
  return erlaubteZeilen;
}
/*
function zeilenAngabenToContainer(welches) {
  if (welches == 1) text = document.getElementById("zeilenErlaubtText").value;
  if (welches == 2) text = document.getElementById("zaehlungErlaubtText").value;
  if (welches == 3) text = document.getElementById("primVielfache").value;
  if (welches == 4) text = document.getElementById("primZahlKreuzRadius").value;

  var zeilenAngaben = new Set();
  text = text.split(",");
  for (var i = 0; i < text.length; i++) {
    text2 = text[i].split("-");

    richtig = true;
    if (text2.length < 3)
      for (var k = 0; k < text2.length; k++)
        if (parseInt(text2[k]) == "NaN") richtig = false;
        else text2[k] = parseInt(text2[k]);
    else richtig = false;

    if (richtig) {
      if (text2.length == 1) text2.push(text2[0]);
      zeilenAngaben.add(text2);
    }
  }
  return zeilenAngaben;
}
*/
/*
function vielfacherAngabentoContainer() {
  text = document.getElementById("VielfacheErlaubtText").value;
  var vielfacherAngaben = new Set();
  text = text.split(",");
  for (var i = 0; i < text.length; i++) {
    text2 = text[i].split("+");
    richtig = true;
    for (var k = 0; k < text2.length; k++)
      if (parseInt(text2[k]) == "NaN") richtig = false;
      else text2[k] = parseInt(text2[k]);
    if (richtig) vielfacherAngaben.add(text2);
  }
  return vielfacherAngaben;
}
*/

function makeAllerlaubteZeilenVielfacher(zeilenAngaben1: Set<number[]>) {
  const zeilenAngaben: number[][] = Array.from(zeilenAngaben1);
  var muls: number[];
  var mul: number
  erlaubteZeilen = new Set();
  var last: number;
  for (var i: number = 0; i < zeilenAngaben.length; i++) {
    last = zeilenAngaben[i][0];
    muls = [];
    mul = 1;
    last = mul * zeilenAngaben[i][0];
    while (last < 1025) {
      muls.push(last);
      last = mul * zeilenAngaben[i][0];
      mul++;
    }
    for (var h: number = 0; h < muls.length; h++) {
      if (zeilenAngaben[i].length == 1) {
        erlaubteZeilen.add(muls[h]);
      } else
        for (var k: number = 1; k < zeilenAngaben[i].length; k++) {
          erlaubteZeilen.add(muls[h] - zeilenAngaben[i][k]);
          //window.alert(parseInt(muls[h]}-zeilenAngaben[i][k]));
          erlaubteZeilen.add(zeilenAngaben[i][k] + muls[h]);
        }
    }
  }
  return erlaubteZeilen;
}

function makeAllerlaubteZeilenPotenzen(zeilenAngaben1: Set<number>) {
  const zeilenAngaben: number[] = Array.from(zeilenAngaben1);
  erlaubteZeilen = new Set();
  var exponent, potenz: number;
  for (var i: number = 0; i < zeilenAngaben.length; i++) {
    if (zeilenAngaben[i] > 0) {
      exponent = 1;
      potenz = Math.pow(zeilenAngaben[i], exponent);
      while (potenz < 1025) {
        erlaubteZeilen.add(potenz);
        potenz = Math.pow(zeilenAngaben[i], exponent);
        //window.alert(potenz);
        exponent++;
      }
    }
  }
  return erlaubteZeilen;
}

function intersection(setA: Set<any>, setB: Set<any>) {
  var _intersection = new Set();
  for (var elem of setB) {
    if (setA.has(elem)) {
      _intersection.add(elem);
    }
  }
  return _intersection;
}

function makeAllAllowedZeilenPrimRichtungen(): Set<number> {
  var innen: boolean = document.getElementById("proInnen").checked;
  var aussen: boolean = document.getElementById("proAussen").checked;
  var hand: boolean = document.getElementById("gegenDritte").checked;
  var faehig: boolean = document.getElementById("proDritte").checked;
  erlaubteZeilen = new Set();
  var inkrement: number;

  if (hand || faehig) {
    if (hand) inkrement = 3;
    else inkrement = 2;
    for (var i: number = 0; i < 1025; i += inkrement) erlaubteZeilen.add(i);
    return erlaubteZeilen;
  }

  if (innen || aussen) {
    var innenAussen: Set<number> = new Set();
    if (aussen) innenAussen = new Set([1, 7, 13, 19]);
    if (innen) innenAussen = new Set([5, 11, 17, 23]);
    var primZahlenModulo: Set<number>;
    var vielfacher: number;

    for (var i: number = 0; i < 1025; i++) {
      primZahlenModulo = new Set();
      for (var k: number = 2; k < primZahlen.length; k += 1) {
        vielfacher = 1;
        while (i / vielfacher > 2) {
          if (primZahlen[k] == i / vielfacher) {
            vielfacher = i;
            primZahlenModulo.add(primZahlen[k] % 24);
          }
          vielfacher++;
        }
      }
      if (intersection(primZahlenModulo, innenAussen).size != 0)
        erlaubteZeilen.add(i);
    }
    return erlaubteZeilen;
  }
}

function makeAllAllowedZeilenHimmelskoerper(): Set<number> {
  const sonneWahl: boolean = document.getElementById("sonneWahl").checked;
  const mondWahl: boolean = document.getElementById("mondWahl").checked;
  const planetWahl: boolean = document.getElementById("planetWahl").checked;
  const schwarzeSonneWahl: boolean = document.getElementById("schwarzeSonneWahl").checked;
  erlaubteZeilen = new Set();
  if (mondWahl) {
    erlaubteZeilen = new Set(alleMonde);
    return erlaubteZeilen;
  }
  if (sonneWahl) {
    const alleMondeSet: Set<number> = new Set(alleMonde);
    for (var i: number = 1; i < 1025; i++) {
      if (!alleMondeSet.has(i)) erlaubteZeilen.add(i);
    }
    return erlaubteZeilen;
  }
  if (planetWahl) {
    for (var i: number = 2; i < 1025; i += 2) erlaubteZeilen.add(i);
    return erlaubteZeilen;
  }
  if (schwarzeSonneWahl) {
    for (var i: number = 3; i < 1025; i += 3) erlaubteZeilen.add(i);
    return erlaubteZeilen;
  }
}
function makeAllowedZeilenFromPrimVielfacher(zeilenAngaben1: Set<number>): Set<number> {
  const zeilenAngaben: number[] = Array.from(zeilenAngaben1);
  erlaubteZeilen = new Set();
  const ersteSpalte: HTMLCollectionOf<HTMLTableCellElement> = document
    .getElementById("bigtable")
    .getElementsByClassName("r_0");
  for (var i = 0; i < 1025; i++)
    for (var k = 0; k < zeilenAngaben.length; k++)
      if (zahlIstVielfacherEinerPrimzahl(i, zeilenAngaben[k]))
        erlaubteZeilen.add(i);
  return erlaubteZeilen;
}

function zahlIstVielfacherEinerPrimzahl(zahl1: number | string, vielfacher1: string | number): boolean {
  const zahl: number = parseInt(zahl1);
  const vielfacher: number = parseInt(vielfacher1);
  if (isNaN(zahl) || isNaN(vielfacher)) return false;

  var stimmt: boolean = false;
  for (var i: number = 0; i < primZahlen.length; i++)
    if (primZahlen[i] == zahl / vielfacher) stimmt = true;
  return stimmt;
}

function makeAllowedZeilenFromZaehlung(zeilenMenge: Set<number>): Set<number> {
  const ersteSpalte: HTMLCollectionOf<HTMLTableCellElement> = document.getElementById("bigtable").getElementsByClassName("r_0");

  //console.log("ersteSpalte", ersteSpalte.length);
  const erlaubteZaehlungen: Set<number> = zeilenMenge;
  erlaubteZeilen = new Set();
  //window.alert(Array.from(erlaubteZaehlungen).join(" "));
  //window.alert(ersteSpalte.length.toString());
  var zaehlung1: string;
  var zaehlung: number;
  var wirklicheZeile1: RegExpMatchArray | null;
  var wirklicheZeile: RegExpMatchArray | string;

  for (var i: number = 0; i < ersteSpalte.length; i++) {
    //window.alert(ersteSpalte[i].getElementsByTagName("label")[0].innerHTML);
    zaehlung1 = ersteSpalte[i].innerHTML.trim();
    if (!isNaN(zaehlung1)) {
        zaehlung = parseInt(zaehlung1);
        //window.alert(zaehlung.toString());
        if (!isNaN(zaehlung) && erlaubteZaehlungen.has(zaehlung)) {
            wirklicheZeile1 = ersteSpalte[i].className.match(/z_\s*(\d+)/g);
            //console.log("wirklicheZeile1", wirklicheZeile1);
            if (wirklicheZeile1 != null) {
                wirklicheZeile = wirklicheZeile1; //.toString();
                //window.alert(ersteSpalte[i].className);
                //window.alert(wirklicheZeile);
                if (wirklicheZeile.length > 0) {
                    wirklicheZeile = wirklicheZeile[0].substr(2);
                    erlaubteZeilen.add(parseInt(wirklicheZeile));
                }
            }
        }
    }
  }
  return erlaubteZeilen;
}

function makeAllAllowedZeilen(zeilenAngaben1: Set<number[]>): Set<number> {
  const zeilenAngaben: number[][] = Array.from(zeilenAngaben1);
  //console.log("zeilenAngaben::-> ",zeilenAngaben)
  erlaubteZeilen = new Set();
  for (var i: number = 0; i < zeilenAngaben.length; i++) {
    for (var k: number = zeilenAngaben[i][0]; k <= zeilenAngaben[i][1]; k++) {
      erlaubteZeilen.add(k);
    }
  }
  return erlaubteZeilen;
}

function makeAllowedZeilenFromPrimZahlKreuzRadius(zeilenAngaben1: Set<number>): number[] {
  const zeilenAngaben: number[] = Array.from(zeilenAngaben1);
  erlaubteZeilen = new Set();
  for (var i: number = 1; i < 1025; i++)
    for (var k: number = 0; k < zeilenAngaben.length; k++)
      if (zeilenAngaben[k] == Math.floor((i - 1) / 24) + 1)
        erlaubteZeilen.add(i);

  return zeilenAngaben;
}

var spalten_r__: Set<number> = new Set();

function get_r__SpaltenNummern() {
  //const tAble:  HTMLTableElement = document.getElementById("bigtable") as HTMLTableElement;
  let tabelenkopfZeile : HTMLCollectionOf<HTMLTableCellElement>;
  tabelenkopfZeile = tdClasses;
  var num1: RegExpMatchArray | null;
  var num: RegExpMatchArray;
  var num2: number;
  //console.log(tAble.rows.length)
  for (var i: number = 0; i < tabelenkopfZeile.length; i++) {
    if (tabelenkopfZeile[i].style.display === "table-cell") {
      num1 = tabelenkopfZeile[i].className.match(/r_(\d+)/);
      if (num1 != null) {
      num = num1
      if (num.length > 1) {
            num2 = parseInt(num[1]);
            spalten_r__.add(num2);

          }
      }
    }
  }
  //console.log(spalten_r__)
}

/*
var verboteneZeilen = [];

function invertErlaubteZeilen() {
    verboteneZeilen = [];
    for (var i=0; i<1025; i++) {
        if ((!i in erlaubteZeilen))
            verboteneZeilen.push(i);
    }
}
*/

function erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(which: number) {
  const Spalten_r__Array: number[] = Array.from(spalten_r__);
  //console.log("erlaubte Zeilen: ", erlaubteZeilen)
  const erlaubteZeilen_Array: number[] = Array.from(erlaubteZeilen);
  const erlaubteZeilen_String: string = erlaubteZeilen_Array.join(",");
  const neuErlauben: boolean = document.getElementsByClassName("neuErlauben")[which].checked;
  const neuHinfort: boolean = document.getElementsByClassName("neuHinfort")[which].checked;
  const dazuErlauben: boolean = document.getElementsByClassName("dazuErlauben")[which].checked;
  const dazuHinfort: boolean = document.getElementsByClassName("dazuHinfort")[which].checked;
  const dazuEinschraenkend: boolean =
    document.getElementsByClassName("dazuEinschraenkend")[which].checked;
  //window.alert(neuErlauben+" "+neuHinfort+" "+dazuErlauben+" "+dazuHinfort);
  const spalte: HTMLCollectionOf<HTMLTableRowElement> = document.getElementById("bigtable").rows;
  var tabellenZelle: HTMLTableRowElement;
  var echteZeilenNummer1: RegExpMatchArray | null;
  var echteZeilenNummer: number;
  for (var s: number = 1; s < spalte.length; s++) {
    tabellenZelle = spalte[s];
    //if (s < 115)
    if (false && s < 115)
      zeilenLetztendlichZeigenVerstecken(
        s,
        neuErlauben,
        dazuErlauben,
        neuHinfort,
        dazuHinfort,
        tabellenZelle,
        dazuEinschraenkend
      );
    else {
      echteZeilenNummer1 = spalte[s].cells[0].className.match(/z_\s*(\d+)/g);
      if (echteZeilenNummer1 != null && echteZeilenNummer1.length > 0) {
        echteZeilenNummer = parseInt(echteZeilenNummer1[0].substr(2));
        zeilenLetztendlichZeigenVerstecken(
          echteZeilenNummer,
          neuErlauben,
          dazuErlauben,
          neuHinfort,
          dazuHinfort,
          tabellenZelle,
          dazuEinschraenkend
        );
      }
    }
  }
}

function zeilenLetztendlichZeigenVerstecken(
  s: number,
  neuErlauben: boolean,
  dazuErlauben: boolean,
  neuHinfort: boolean,
  dazuHinfort: boolean,
  tabellenZelle: HTMLTableRowElement,
  dazuEinschraenkend: boolean
) {
  if (
    ((erlaubteZeilen.has(s) && (neuErlauben || dazuErlauben)) ||
      (!erlaubteZeilen.has(s) && neuHinfort)) &&
    !dazuHinfort
  ) {
    tabellenZelle.style.display = "table-row";
    //animateAllPolygons();
  } else if (
    ((neuErlauben || neuHinfort) && !dazuErlauben) ||
    (dazuHinfort && erlaubteZeilen.has(s)) ||
    (dazuEinschraenkend && !erlaubteZeilen.has(s))
  )
    tabellenZelle.style.display = "none";
}

function clickPotenzenErlaubenUsw() {
  makeAllerlaubteZeilenPotenzen(zeilenAngabenToMengeDirekt(6));
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(2);
}

function clickVielfacheErlaubenUsw() {
  //makeAllerlaubteZeilenVielfacher(vielfacherAngabentoContainer());
  zeilenAngabenToMengeDirekt(5, true);
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(1);
}

function clickHimmelskoerperErlaubenUsw() {
  erlaubteZeilen = makeAllAllowedZeilenHimmelskoerper();
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(3);
}

function clickZeilenErlaubenUsw() {
  zeilenAngabenToMengeDirekt(1);
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(0);
}

function clickZaehlungenErlaubenUsw() {
  makeAllowedZeilenFromZaehlung(zeilenAngabenToMengeDirekt(2));
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(4);
}
function clickPrimVielfacheErlaubenUsw() {
  makeAllowedZeilenFromPrimVielfacher(zeilenAngabenToMengeDirekt(3));
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(5);
}
function clickPrimRichtungenErlaubenUsw() {
  erlaubteZeilen = makeAllAllowedZeilenPrimRichtungen();
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(6);
}

function clickPrimZahlKreuzRadiusErlaubenUsw() {
  makeAllowedZeilenFromPrimZahlKreuzRadius(zeilenAngabenToMengeDirekt(4));
  get_r__SpaltenNummern();
  erlaubeVerbieteZeilenBeiZeilenErlaubenVerbieten(7);
}
