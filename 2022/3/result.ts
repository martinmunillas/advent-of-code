import { readFileSync } from "fs";
import { join } from "path";
import { getPrioritiesPointsPartA, getPrioritiesPointsPartB } from ".";

const input = readFileSync(join(__dirname, "./input/input.txt"), "utf-8");
console.log("Result A:", getPrioritiesPointsPartA(input));
console.log("Result B:", getPrioritiesPointsPartB(input));
