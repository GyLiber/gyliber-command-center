// Pure finite metric example, reviewed by Sol. No ambient state or rendering.
export const points = Object.freeze([1, 3, 4]);
export function distance(a, b) {
  if (!points.includes(a) || !points.includes(b)) throw new RangeError('Choose a point in {1, 3, 4}.');
  return Math.abs(a - b);
}
export function evaluate({ x = 1, y = 3, z = 4 } = {}) {
  const direct = distance(x, z), first = distance(x, y), second = distance(y, z);
  return Object.freeze({ x, y, z, direct, first, second, detour: first + second,
    slack: first + second - direct, equality: direct === first + second });
}
