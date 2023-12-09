const getCharPoints = (char: string) => {
  const charCode = char.charCodeAt(0);
  if (charCode >= 97) {
    return charCode - 96;
  } else {
    return charCode - 38;
  }
};

export const getPrioritiesPointsPartA = (input: string): number => {
  let points = 0;
  let currentLine = "";
  i: for (let i = 0; i <= input.length; i++) {
    const char = input[i];
    if (char === "\n" || i === input.length) {
      let repeated = "";
      let isInFirstHalf: Record<string, boolean> = {};
      j: for (let j = 0; j < currentLine.length; j++) {
        if (j < currentLine.length / 2) {
          isInFirstHalf[currentLine[j]] = true;
        } else if (isInFirstHalf[currentLine[j]]) {
          repeated = currentLine[j];
          break j;
        }
      }
      currentLine = "";
      points += getCharPoints(repeated);
      continue i;
    }
    currentLine += char;
  }

  return points;
};

export const getPrioritiesPointsPartB = (input: string): number => {
  let points = 0;
  let currentLine = "";
  let currentGroup = [];
  i: for (let i = 0; i <= input.length + 1; i++) {
    const char = input[i];
    if (char === "\n" || i === input.length) {
      currentGroup.push(currentLine);
      currentLine = "";
      continue i;
    }
    if (currentGroup.length === 3) {
      let badge = "";
      const count: Record<string, number> = {};
      const [firstLine, secondLine, thirdLine] = currentGroup;
      for (let j = 0; j < firstLine.length; j++) {
        const char = firstLine[j];
        count[char] = 1;
      }
      for (let j = 0; j < secondLine.length; j++) {
        const char = secondLine[j];
        if (count[char]) {
          count[char] = 2;
        }
      }
      j: for (let j = 0; j < thirdLine.length; j++) {
        const char = thirdLine[j];
        if (count[char] == 2) {
          badge = char;
          break j;
        }
      }
      points += getCharPoints(badge);
      currentGroup = [];
    }
    currentLine += char;
  }

  return points;
};
