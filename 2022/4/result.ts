import { readFileSync } from "fs";
import { join } from "path";
import {
  getContainingSectionsCountPartA,
  getContainingSectionsCountPartB,
} from ".";

const input = readFileSync(join(__dirname, "./input/input.txt"), "utf-8");

console.log("Result A", getContainingSectionsCountPartA(input));
console.log("Result B", getContainingSectionsCountPartB(input));
