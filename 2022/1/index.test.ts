import { readFileSync } from "fs";
import { getMostCalories, getTop3CaloriesAddedUp } from ".";
import { resolve } from "path";

const test1 = readFileSync(resolve(__dirname, "./input/example1.txt"), "utf-8");
const test2 = readFileSync(resolve(__dirname, "./input/example2.txt"), "utf-8");

describe("First day part a examples", () => {
  it("should retrieve the 24000 calories", () => {
    expect(getMostCalories(test1)).toBe(24000);
  });

  it("should retrieve the 161 calories", () => {
    expect(getMostCalories(test2)).toBe(161);
  });
});

describe("First day part b examples", () => {
  it("should retrieve the 24000 calories", () => {
    expect(getTop3CaloriesAddedUp(test1)).toBe(45000);
  });

  it("should retrieve the 161 calories", () => {
    expect(getTop3CaloriesAddedUp(test2)).toBe(274);
  });
});
