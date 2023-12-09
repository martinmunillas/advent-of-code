import { readFileSync } from "fs";
import { join } from "path";
import { getHighestScenicScore, getVisibleTrees } from ".";

const input = readFileSync(join(__dirname, "./input/input.txt"), "utf-8");

console.log("Result A:", getVisibleTrees(input));
console.log("Result B:", getHighestScenicScore(input));
