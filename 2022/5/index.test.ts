import { readFileSync } from "fs";
import { join } from "path";
import { getTopOfFinalStacksPartA, getTopOfFinalStacksPartB } from ".";

const example = readFileSync(join(__dirname, "./input/example.txt"), "utf-8");

describe("Day 4 part A", () => {
  it("should be CMZ", () => {
    expect(getTopOfFinalStacksPartA(example)).toBe("CMZ");
  });
});

describe("Day 4 part B", () => {
  it("should be CMZ", () => {
    expect(getTopOfFinalStacksPartB(example)).toBe("MCD");
  });
});
