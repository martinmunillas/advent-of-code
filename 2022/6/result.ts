import { readFileSync } from "fs";
import { join } from "path";
import { getMarkerPartA, getMarkerPartB } from ".";

const input = readFileSync(join(__dirname, "./input.txt"), "utf-8");
console.log("Result A:", getMarkerPartA(input));
console.log("Result B:", getMarkerPartB(input));
