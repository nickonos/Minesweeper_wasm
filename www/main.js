import init, {Minefield, Position} from "../pkg/Minesweeper_wasm.js";

class MinesweeperClass{
    constructor() {
        this.initialize()
    }

    initialize = async () => {
        await init()

        this.generateMinefield(10, 8, 80)
    }

    generateMinefield(width, height, bombs){
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

        this.Minefield = Minefield.new(width, height, bombs);
    }

    updateMinefield(){
        let array = this.Minefield.get_minefield()

        array.map((item, key) => {
            let cell = document.getElementById("Cell-" + Math.floor(key / 30) + "-" + key % 30)

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