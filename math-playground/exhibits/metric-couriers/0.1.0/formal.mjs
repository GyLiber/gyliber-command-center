export const concept = Object.freeze({
  "title": "Metric couriers",
  "kind": "metric",
  "concept_type": "definition",
  "notation": "X ≠ ∅; d: X × X → [0, ∞). The pair (X, d) is a metric space.",
  "hypotheses": "The conditions below hold for every x, y, z ∈ X. X need not be a set of Euclidean points.",
  "statement": "For every x, y, z ∈ X:\n(1) d(x,y) = 0 if and only if x = y;\n(2) d(x,y) = d(y,x);\n(3) d(x,z) ≤ d(x,y) + d(y,z).\nNonnegativity is required by the codomain [0, ∞).",
  "visual_mapping": "A finite representative example: X = {1,3,4}, d(a,b) = |a−b|, restricted from the usual metric on ℝ. Choose a start, a via point and a finish. Direct and detour lengths come from the engine.",
  "limitations": "This finite example does not replace the general definition or prove the axioms on all real numbers. Vertical lanes, faces and cosmetic movement assert no additional mathematics.",
  "source_file": "Sol-authored original synthetic example",
  "source_quote": "No private course quotation is published. Original synthetic test: docs/examples/metric-spaces-test.tex.",
  "palette": "ocean"
});
