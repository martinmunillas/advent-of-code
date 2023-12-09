import { readFileSync } from "fs";
import { join } from "path";
import {
  getContainingSectionsCountPartA,
  getContainingSectionsCountPartB,
} from ".";

const example = readFileSync(join(__dirname, "./input/example.txt"), "utf-8");

describe("Day 4 part A", () => {
  it("should sum be 2", () => {
    expect(getContainingSectionsCountPartA(example)).toBe(2);
  });
});

describe("Day 4 part B", () => {
  it("should be 4", () => {
    expect(getContainingSectionsCountPartB(example)).toBe(4);
  });
});
