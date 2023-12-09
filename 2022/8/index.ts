export const getVisibleTrees = (input: string): number => {
  const size = Math.floor(Math.sqrt(input.length));
  const idx = (i: number, j: number) => i * (size + 1) + j;
  let result = 0;
  for (let i = 1; i < size - 1; i++) {
    for (let j = 1; j < size - 1; j++) {
      let hidden = 0;
      let k = j - 1;
      const currentTree = input[idx(i, j)];
      k: while (k > -1) {
        if (input[idx(i, k)] >= currentTree) {
          hidden++;
          break k;
        }
        k--;
      }
      k = j + 1;
      k: while (k < size) {
        if (input[idx(i, k)] >= currentTree) {
          hidden++;
          break k;
        }
        k++;
      }
      k = i - 1;
      k: while (k > -1) {
        if (input[idx(k, j)] >= currentTree) {
          hidden++;
          break k;
        }
        k--;
      }
      k = i + 1;
      k: while (k < size) {
        if (input[idx(k, j)] >= currentTree) {
          hidden++;
          break k;
        }
        k++;
      }
      if (hidden < 4) {
        result++;
      }
    }
  }
  return result + size * 2 + (size - 2) * 2;
};
export const getHighestScenicScore = (input: string): number => {
  const size = Math.floor(Math.sqrt(input.length));
  const idx = (i: number, j: number) => i * (size + 1) + j;
  let result = 0;
  for (let i = 1; i < size - 1; i++) {
    for (let j = 1; j < size - 1; j++) {
      let k = j - 1;
      const currentTree = input[idx(i, j)];
      let left = 0;
      let right = 0;
      let up = 0;
      let down = 0;
      k: while (k > -1) {
        left++;
        if (input[idx(i, k)] >= currentTree) {
          break k;
        }
        k--;
      }
      k = j + 1;
      k: while (k < size) {
        right++;
        if (input[idx(i, k)] >= currentTree) {
          break k;
        }
        k++;
      }
      k = i - 1;
      k: while (k > -1) {
        up++;
        if (input[idx(k, j)] >= currentTree) {
          break k;
        }
        k--;
      }
      k = i + 1;
      k: while (k < size) {
        down++;
        if (input[idx(k, j)] >= currentTree) {
          break k;
        }
        k++;
      }
      result = Math.max(result, left * right * up * down);
    }
  }
  return result;
};
