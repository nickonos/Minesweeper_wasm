import init, {Minefield, Position} from "../pkg/Minesweeper_wasm.js";

class MinesweeperClass{
    constructor() {
        this.initialize().then(_ => {})
    }

    initialize = async () => {
        await init()
        this.width = 10;
        this.height = 10;
        this.bombs = 99;

        this.generateMinefield(this.width, this.height)
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

    updateMinefield(){
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

    }

    onCellClick(e){


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
                console.log("Failed")
            }
        }else{
             this.Minefield.flag_field(Position.new( parseInt(x) , parseInt(y)))
        }

        this.updateMinefield();
    }
}

let ms = new MinesweeperClass()