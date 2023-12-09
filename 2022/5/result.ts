import { readFileSync } from "fs";
import { join } from "path";
import { getTopOfFinalStacksPartA, getTopOfFinalStacksPartB } from ".";

const input = readFileSync(join(__dirname, "./input/input.txt"), "utf-8");

console.log("Result A", getTopOfFinalStacksPartA(input));
console.log("Result B", getTopOfFinalStacksPartB(input));
