export const config = {"id":"b93bdc8cd55870c2fbcf20d0d63e5dba","kind":"circle","palette":"candy"};
// Reviewed pure mathematical engine. No DOM, network, storage or implicit clock.
export const engineVersion = '0.1.0';
const clamp = (value, low, high, fallback) => Number.isFinite(Number(value))
  ? Math.max(low, Math.min(high, Number(value))) : fallback;
export function factorial(n) {
  if (!Number.isInteger(n) || n < 0 || n > 6) throw new RangeError('n must be an integer from 0 to 6');
  let result = 1;
  for (let i = 2; i <= n; i++) result *= i;
  return result;
}
export function permutation(n, index) {
  const count = factorial(n);
  if (!Number.isInteger(index) || index < 0 || index >= count) throw new RangeError('permutation index out of range');
  const remaining = Array.from({ length: n }, (_, i) => i);
  const output = [];
  for (let slots = n; slots > 0; slots--) {
    const block = factorial(slots - 1);
    output.push(remaining.splice(Math.floor(index / block), 1)[0]);
    index %= block;
  }
  return output;
}
export function evaluate(input = {}, phase = 0) {
  const t = clamp(phase, 0, 1, 0);
  switch (config.kind) {
    case 'circle': {
      const radius = clamp(input.radius, 0.5, 3, 1.5);
      const unwrap = clamp(input.unwrap, 0, 1, 0);
      const circumference = 2 * Math.PI * radius;
      return { kind: 'circle', radius, unwrap, circumference, diameter: 2 * radius,
        ratio: Math.PI, ribbonLength: circumference * unwrap, phase: t };
    }
    case 'sequence': {
      const epsilon = clamp(input.epsilon, 0.02, 0.5, 0.15);
      const n = Math.round(clamp(input.n, 1, 100, 10));
      const threshold = Math.floor(1 / epsilon) + 1;
      return { kind: 'sequence', epsilon, n, threshold, value: 1 / n,
        inside: 1 / n < epsilon, terms: Array.from({ length: 50 }, (_, i) => 1 / (i + 1)), phase: t };
    }
    case 'permutation': {
      const n = Math.round(clamp(input.n, 2, 6, 4));
      const count = factorial(n);
      const index = Math.round(clamp(input.index, 0, count - 1, 0));
      return { kind: 'permutation', n, count, index, order: permutation(n, index), phase: t };
    }
    case 'metaphor':
      return { kind: 'metaphor', spread: clamp(input.spread, 0, 1, 0.5), phase: t };
    default: throw new TypeError('unsupported scene');
  }
}
