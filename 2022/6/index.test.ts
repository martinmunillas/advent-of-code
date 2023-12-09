import { getMarkerPartA, getMarkerPartB } from ".";

const example1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
const example2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const example3 = "nppdvjthqldpwncqszvftbrmjlhg";
const example4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
const example5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

describe("Day 6 partA", () => {
  it("should sum all the prioritize and return 7", () => {
    expect(getMarkerPartA(example1)).toBe(7);
  });

  it("should sum all the prioritize and return 5", () => {
    expect(getMarkerPartA(example2)).toBe(5);
  });
  it("should sum all the prioritize and return 6", () => {
    expect(getMarkerPartA(example3)).toBe(6);
  });
  it("should sum all the prioritize and return 10", () => {
    expect(getMarkerPartA(example4)).toBe(10);
  });
  it("should sum all the prioritize and return 11", () => {
    expect(getMarkerPartA(example5)).toBe(11);
  });
});

describe("Day 6 part B", () => {
  it("should sum all the prioritize and return 19", () => {
    expect(getMarkerPartB(example1)).toBe(19);
  });

  it("should sum all the prioritize and return 23", () => {
    expect(getMarkerPartB(example2)).toBe(23);
  });
  it("should sum all the prioritize and return 23", () => {
    expect(getMarkerPartB(example3)).toBe(23);
  });
  it("should sum all the prioritize and return 29", () => {
    expect(getMarkerPartB(example4)).toBe(29);
  });
  it("should sum all the prioritize and return 26", () => {
    expect(getMarkerPartB(example5)).toBe(26);
  });
});
