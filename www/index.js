import * as guessing from "sample";

const min = 1;
const max = 100;
const input = document.getElementById("input");
const result = document.getElementById("result");
let g;

const reset = () => {
	let n = Math.floor(Math.random() * (max + 1 - min)) + min;
    g = new guessing.Guessing(n);
    input.value = "";
    result.innerHTML = "";
}

document.getElementById("submit").addEventListener("click", () => {
    const r = g.check(parseInt(input.value));

    if (r === guessing.Result.Equal){
        alert("正解");
        reset();
    }else if (r === guessing.Result.Less){
        result.innerHTML += input.value + "は正解より大きいです<br>";
    }else{
        result.innerHTML += input.value + "は正解より小さいです<br>";
    }

    input.value = "";
});

reset()