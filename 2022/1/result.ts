import { readFileSync } from "fs";
import { getMostCalories, getTop3CaloriesAddedUp } from ".";
import { resolve } from "path";

const input = readFileSync(resolve(__dirname, "./input/input.txt"), "utf-8");

console.log("Result A:", getMostCalories(input));

console.log("Result B:", getTop3CaloriesAddedUp(input));
