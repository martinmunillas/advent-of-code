const translations = {
  X: "A",
  Y: "B",
  Z: "C",
};

const winnings = {
  A: "C",
  B: "A",
  C: "B",
};

const points = {
  A: 1,
  B: 2,
  C: 3,
};

export const getResultPartA = (input: string): number => {
  const lines = input.length / 4;
  let totalResult = 0;
  for (let line = 0; line < lines; line++) {
    const opponent = input[line * 4] as keyof typeof points;
    const ours = translations[
      input[line * 4 + 2] as keyof typeof translations
    ] as keyof typeof points;

    let result = 0;
    if (opponent === ours) {
      result = 3 + points[ours];
    } else if (winnings[opponent] === ours) {
      result = points[ours];
    } else {
      result = 6 + points[ours];
    }
    totalResult += result;
  }
  return totalResult;
};

const lose = {
  A: "B",
  B: "C",
  C: "A",
};

export const getResultPartB = (input: string): number => {
  const lines = input.length / 4;
  let totalResult = 0;
  for (let line = 0; line < lines; line++) {
    const opponent = input[line * 4] as keyof typeof points;
    const result = input[line * 4 + 2] as keyof typeof translations;
    switch (result) {
      case "X":
        totalResult += points[winnings[opponent] as keyof typeof points];
        break;
      case "Y":
        totalResult += 3 + points[opponent];
        break;
      case "Z":
        totalResult += 6 + points[lose[opponent] as keyof typeof points];
        break;
    }
  }
  return totalResult;
};
