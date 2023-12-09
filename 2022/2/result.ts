import { readFileSync } from "fs";
import { resolve } from "path";
import { getResultPartA, getResultPartB } from ".";

const input = readFileSync(resolve(__dirname, "./input/input.txt"), "utf-8");

console.log("Result A:", getResultPartA(input));
console.log("Result B:", getResultPartB(input));
