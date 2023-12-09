import { readFileSync } from "fs";
import { getHighestScenicScore, getVisibleTrees } from ".";
import { join } from "path";

const example = readFileSync(join(__dirname, "./input/example.txt"), "utf-8");

describe("Day 8 part A", () => {
  it("should get 21 visible trees", () => {
    expect(getVisibleTrees(example)).toBe(21);
  });
});

describe("Sixteenth challenge", () => {
  it("should get 8 scenic score", () => {
    expect(getHighestScenicScore(example)).toBe(8);
  });
});
