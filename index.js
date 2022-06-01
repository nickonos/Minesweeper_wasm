import init, {Minefield} from "./pkg/Minesweeper_wasm.js";
import {memory} from "./pkg/Minesweeper_wasm_bg";

class MinesweeperClass{
    constructor() {
        this.initialize()
    }

    initialize = async () => {
        await init()
        this.Minefield = Minefield.new(30, 16, 80);

        console.log(this.Minefield)
        console.log(memory)
        let container = document.getElementById("minesweeper-container")

        /*this.Minefield.map((item, y_key) => {

            let cellContainer = document.createElement("div")
            cellContainer.setAttribute("id", "Cell-Container-"+ y_key)
            cellContainer.setAttribute("class", "minesweeper-cell-container")
            container.appendChild(cellContainer)

            item.map((cell, x_key) => {
                let div = document.createElement("div")
                div.setAttribute("id", "Cell-"+ y_key + "-" + x_key)
                div.setAttribute("class", "minesweeper-cell")
                div.onclick = (e => this.onCellClick(e))
                cellContainer.appendChild(div)
            })

        })*/
    }

    onCellClick(e){
        let isFlag = e.ctrlKey

        let Cell = e.target

        let arr = Cell.id.split('-')

        let y = arr[1]
        let x = arr[2]
        let input = {
            fields : this.MinesweeperArray,
            x: x,
            y: y
        }
        if (!isFlag){
            ClickField(input)
        }

        console.log(y,x)
        console.log(e)
    }
}

let ms = new MinesweeperClass()