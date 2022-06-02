import init, {Minefield, Position} from "../pkg/Minesweeper_wasm.js";

class MinesweeperClass{
    constructor() {
        this.initialize()
    }

    initialize = async () => {
        await init()
        console.log("test")
        this.Minefield = Minefield.new(30, 16, 80);

        let mines = this.Minefield.get_minefield();
        let container = document.getElementById("minesweeper-container")

        let cell_container;
        mines.map((item, y_key) => {

            if (y_key % 30 === 0 || y_key === 0){
                let cellContainer = document.createElement("div")
                cellContainer.setAttribute("id", "Cell-Container-"+ Math.floor(y_key/30))
                cellContainer.setAttribute("class", "minesweeper-cell-container")
                container.appendChild(cellContainer)
                cell_container = cellContainer;
            }

                let div = document.createElement("div")
                div.setAttribute("id", "Cell-"+ Math.floor(y_key/30)  + "-" + y_key % 30)
                div.setAttribute("class", "minesweeper-cell")
                div.onclick = (e => this.onCellClick(e))
                cell_container.appendChild(div)
        })
    }

    updateMinefield(){
        let array = this.Minefield.get_minefield()
        array.map((item, key) => {
            let cell = document.getElementById("Cell-" + Math.floor(key/30)  + "-" + key % 30)

            if (item === 11 && !cell.classList.contains("flagged")){
                cell.classList.add("flagged")
            }

            if (item !== 11 && cell.classList.contains("flagged")){
                cell.classList.remove("flagged")
            }

            if (item === 12){
                cell.classList.add("bomb")
            }

            if (item < 10){
                cell.textContent = item.toString();
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