import init, {Minefield, Position} from "../pkg/Minesweeper_wasm.js";

class MinesweeperClass{
    constructor() {
        this.initialize().then(_ => {})
    }

    initialize = async () => {
        await init()

        this.width = document.getElementById("width-slider").value ?? 16;
        this.height = document.getElementById("height-slider").value ?? 16;
        this.bombs = document.getElementById("bombs-slider").value ?? 40;
        this.failed = false;
        this.Minefield = undefined;
        if (this.interval){
            window.clearInterval(this.interval);
        }

        this.initHTML();

        this.removeMinefield();

        this.generateMinefield(this.width, this.height);
    }

    removeMinefield(){
        let minefield = document.getElementById("minefield");

        while (minefield.firstChild){
            minefield.removeChild(minefield.lastChild)
        }
    }

    generateMinefield(width, height){
        let container = document.getElementById("minefield");

        for (let y = 0; y < height; y++){
            let cellContainer = document.createElement("div");
            cellContainer.setAttribute("id", "Cell-Container-"+ y);
            cellContainer.setAttribute("class", "minesweeper-cell-container");
            container.appendChild(cellContainer);

            for (let x =0; x < width; x++){
                let div = document.createElement("div");
                div.setAttribute("id", "Cell-"+ y  + "-" + x);
                div.setAttribute("class", "minesweeper-cell");
                div.onclick = (e => this.onCellClick(e));
                cellContainer.appendChild(div);
            }
        }
    }

    initHTML(){
        let button = document.getElementById("start-button");

        if (!button)
            console.error("Error finding start button")

        button.onclick = async () => {
            await this.initialize()
        }

        let updateButton = document.getElementById("update-settings");

        if (!updateButton)
            console.error("Error finding update button")

        updateButton.onclick = async () => {
            await this.initialize();
            document.getElementById("settings-overlay").style.display = "none";
        }

        document.getElementById("bomb-counter").innerHTML = ((this.height * this.width) - this.bombs).toString()
        document.getElementById("time-counter").innerHTML = "0";
    }

    updateMinefield(x, y){
        let array = this.Minefield.get_minefield()

        array.map((item, key) => {
            let cell = document.getElementById("Cell-" + Math.floor(key / this.width) + "-" + key % this.width)

            if (!cell)
                return;

            if (item === 11 && !cell.classList.contains("flagged")) {
                cell.classList.add("flagged")
            }

            if (item !== 11 && cell.classList.contains("flagged")) {
                cell.classList.remove("flagged")
            }

            if (item === 12) {
                cell.classList.add("bomb")
            }

            if (item < 10 &&
                !cell.classList.contains("type0") &&
                !cell.classList.contains("type1") &&
                !cell.classList.contains("type2") &&
                !cell.classList.contains("type3") &&
                !cell.classList.contains("type4") &&
                !cell.classList.contains("type5") &&
                !cell.classList.contains("type6") &&
                !cell.classList.contains("type7") &&
                !cell.classList.contains("type8")
            ) {
                cell.classList.add("type" + item.toString());
                this.updateScore();
            }
        })

        if(this.failed){
            let cell = document.getElementById("Cell-" + y + "-" + x)
            cell.classList.remove("bomb")
            cell.classList.add("red-bomb")
        }

    }

    updateTimer(){
        let timer = document.getElementById("time-counter");

        let time = parseInt(timer.innerHTML);
        time++;

        timer.innerHTML = time.toString();
    }

    updateScore(){
        let counter = document.getElementById("bomb-counter");

        let count = parseInt(counter.innerHTML);
        count--;

        counter.innerText = count.toString();
    }

    onCellClick(e){
        if (this.failed)
            return;

        let isFlag = e.ctrlKey

        let Cell = e.target

        let arr = Cell.id.split('-')

        let y = arr[1]
        let x = arr[2]

        if (!this.Minefield){
            this.Minefield = Minefield.new(this.width, this.height, this.bombs, Position.new( parseInt(x) , parseInt(y)));
            this.interval = window.setInterval(this.updateTimer, 1000);
        }

        if (!isFlag){
            let success = this.Minefield.click_field(Position.new( parseInt(x) , parseInt(y)))

            if (!success){
                this.failed = true;
            }
        }else{
             this.Minefield.flag_field(Position.new( parseInt(x) , parseInt(y)))
        }

        this.updateMinefield(x, y);
    }
}

let ms = new MinesweeperClass()
