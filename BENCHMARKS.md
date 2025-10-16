# Accuracy for compile time polynomial evaluations

| Polynomial Equation | Accuracy |
|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------|
| `1.2 * x² * y⁻¹ * z⁰ +` <br> `-0.8 * x³ * y¹ * z⁻² +` <br> `2.5 * x⁻³ * y⁴ * z¹ +` <br> `-1.1 * x⁰ * y⁻² * z³ +` <br> `0.9 * x¹ * y² * z⁻¹` | 1e-9     |
| <br> `1.5 * sin(x₀) * x₁ * x₂² +` <br> `-2.0 * cos(x₀) * x₁³ * x₂ +` <br> `0.5 * exp(x₀) * ln(x₁) * sqrt(x₂)` | 1e-10 | 
| <br> `3.0 * x₀ * sin(x₁) * x₂² * cos(x₃) +` <br> `-1.2 * x₀³ * tan(x₁) * exp(x₂) * x₃ +` <br> `0.7 * ln(x₀) * sqrt(x₁) * atan(x₂) * sinh(x₃) +` <br> `1.1 * cosh(x₀) * x₁ * x₂ * sin(x₃)` | 1e-7  | 
| <br> `2.0 * sin(x₀) * cos(x₁) * x₂² * x₃ * ln(x₄) +` <br> `-3.3 * exp(x₀) * x₁³ * tan(x₂) * sqrt(x₃) * x₄ +` <br> `1.7 * atan(x₀) * sinh(x₁) * cosh(x₂) * x₃ * x₄ +` <br> `0.9 * x₀ * x₁ * x₂ * x₃ * x₄ +` <br> `-0.5 * x₀ * x₁ * x₂ * x₃ * x₄` | 1e-12 |
| <br> `1.1 * cos(x₀) * exp(x₁) * x₂ * ln(x₃) * sqrt(x₄) +` <br> `-2.2 * x₀² * sin(x₁) * x₂ * cosh(x₃) * x₄³ +` <br> `0.8 * tan(x₀) * atan(x₁) * sinh(x₂) * x₃² * x₄ +` <br> `-0.7 * x₀ * ln(x₁) * cos(x₂) * x₃ * exp(x₄) +` <br> `1.5 * x₀ * x₁ * x₂ * x₃ * x₄` | 1e-10 |
| <br> `-1.3 * sinh(x₀) * x₁³ * x₂ * ln(x₃) * cos(x₄) +` <br> `2.4 * exp(x₀) * x₁ * sin(x₂) * cosh(x₃) * x₄² +` <br> `-0.9 * atan(x₀) * x₁ * x₂² * tan(x₃) * sqrt(x₄) +` <br> `1.0 * x₀ * x₁ * x₂ * x₃ * x₄ +` <br> `-0.6 * x₀ * cos(x₁) * exp(x₂) * x₃³ * sin(x₄)` | 1e-7 |
| <br> `2.2 * x₀ * sqrt(x₁) * ln(x₂) * sin(x₃) * exp(x₄) +` <br> `-1.8 * x₀³ * cos(x₁) * x₂ * tan(x₃) * x₄² +` <br> `1.3 * cosh(x₀) * x₁ * x₂ * atan(x₃) * x₄ +` <br> `-0.4 * x₀ * x₁² * sinh(x₂) * x₃³ * cos(x₄) +` <br> `0.7 * x₀ * x₁ * x₂ * x₃ * x₄` | 1e-8 |

----------------------------

**Note:** Lowest accuracy out of all evaluations is 1e-7 (0.0000001).