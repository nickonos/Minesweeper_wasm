import init, {Minefield, Position} from "../pkg/Minesweeper_wasm.js";

class MinesweeperClass{
    constructor() {
        this.initialize().then(_ => {})
    }

    initialize = async () => {
        await init()

        this.initStartButton()

        this.width = 10;
        this.height = 10;
        this.bombs = 20;
        this.failed = false;
        this.Minefield = undefined;

        this.removeMinefield()


        this.generateMinefield(this.width, this.height)
    }

    removeMinefield(){
        let minefield = document.getElementById("minefield")

        while (minefield.firstChild){
            minefield.removeChild(minefield.lastChild)
        }
    }

    generateMinefield(width, height){
        let container = document.getElementById("minefield")

        for (let y = 0; y < height; y++){
            let cellContainer = document.createElement("div")
            cellContainer.setAttribute("id", "Cell-Container-"+ y)
            cellContainer.setAttribute("class", "minesweeper-cell-container")
            container.appendChild(cellContainer)

            for (let x =0; x < width; x++){
                let div = document.createElement("div")
                div.setAttribute("id", "Cell-"+ y  + "-" + x)
                div.setAttribute("class", "minesweeper-cell")
                div.onclick = (e => this.onCellClick(e))
                cellContainer.appendChild(div)
            }
        }


    }

    initStartButton(){
        let button = document.getElementById("start-button")

        if (!button)
            console.error("Error finding start button")

        button.onclick = async () => {
            await this.initialize()
        }
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

            if (item < 10) {
                cell.classList.add("type" + item.toString());
            }
        })

        if(this.failed){
            let cell = document.getElementById("Cell-" + y + "-" + x)
            cell.classList.remove("bomb")
            cell.classList.add("red-bomb")
        }

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
