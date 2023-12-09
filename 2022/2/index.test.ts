import { readFileSync } from "fs";
import { getResultPartA, getResultPartB } from ".";
import { resolve } from "path";

const test1 = readFileSync(resolve(__dirname, "./input/example1.txt"), "utf-8");
const test2 = readFileSync(resolve(__dirname, "./input/example2.txt"), "utf-8");
const test3 = readFileSync(resolve(__dirname, "./input/example3.txt"), "utf-8");

describe("Second day part A", () => {
  it("should result 15", () => {
    expect(getResultPartA(test1)).toBe(15);
  });

  it("should result 24", () => {
    expect(getResultPartA(test2)).toBe(24);
  });

  it("should result 62", () => {
    expect(getResultPartA(test3)).toBe(71);
  });
});

describe("Second day part B", () => {
  it("should result 12", () => {
    expect(getResultPartB(test1)).toBe(12);
  });
});
