import { readFileSync } from "fs";
import { getDirsUnder100000PartA, getDirsUnder100000PartB } from ".";
import { join } from "path";

const example = readFileSync(join(__dirname, "./input/example.txt"), "utf-8");

describe("Day 7 part A", () => {
  it("should sum all directories with size under 100000", () => {
    expect(getDirsUnder100000PartA(example)).toBe(95437);
  });
});

describe("Day 7 part A", () => {
  it("should sum all directories with size under 100000", () => {
    expect(getDirsUnder100000PartB(example)).toBe(24933642);
  });
});
