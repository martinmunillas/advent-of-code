import { readFileSync } from "fs";
import { join } from "path";
import { getDirsUnder100000PartA, getDirsUnder100000PartB } from ".";

const input = readFileSync(join(__dirname, "./input/input.txt"), "utf-8");
console.log("Result A:", getDirsUnder100000PartA(input));
console.log("Result B:", getDirsUnder100000PartB(input));
