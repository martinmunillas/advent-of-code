import { readFileSync } from "fs";
import { join } from "path";
import { getPrioritiesPointsPartA, getPrioritiesPointsPartB } from ".";

const example = readFileSync(join(__dirname, "./input/example.txt"), "utf-8");
describe("Third day part A", () => {
  it("should sum all the prioritize and return 157", () => {
    expect(getPrioritiesPointsPartA(example)).toBe(157);
  });
});

describe("Third day part B", () => {
  it("should sum all the prioritize and return 70", () => {
    expect(getPrioritiesPointsPartB(example)).toBe(70);
  });
});
