# Benchmarks for Polynomial Term Evaluations

| Polynomial                                              | Approximation Error            | Notes                                                   |
|---------------------------------------------------------|-------------------------------|---------------------------------------------------------|
| 3 * sin(a)                                              | < 1e-6                        | Single-variable sin term                                |
| 2 * sin(a) * sin(b)                                     | < 1e-6                        | Two-variable sin term                                   |
| sin(a) * sin(b) * sin(c)                                | < 1e-6                        | Three-variable sin term                                 |
| 5 * cos(a)                                              | < 1e-6                        | Single-variable cos term                                |
| 2 * cos(a) * cos(b)                                     | < 1e-6                        | Two-variable cos term                                   |
| cos(a) * cos(b) * cos(c)                                | < 1e-6                        | Three-variable cos term                                 |
| 1.5 * exp(a)                                           | < 1e-6                        | Single-variable exponential term                        |
| 2 * exp(a) * exp(b)                                     | < 1e-6                        | Two-variable exponential term                           |
| 3 * tan(a)                                              | < 1e-6                        | Single-variable tan term                                |
| 2 * tan(a) * tan(b)                                     | < 1e-6                        | Two-variable tan term                                   |
| 2^3                                                    | < 1e-6                        | Static power function (single variable)                 |
| 3 * (a^1) * (b^2)                                       | < 1e-6                        | Static power function (two variables)                   |
| sqrt(a)                                                | < 1e-6                        | Single-variable sqrt term                               |
| 2 * sqrt(a)                                            | < 1e-6                        | Single-variable sqrt term with zero input              |
| sinh(a)                                                | < 1e-6                        | Single-variable hyperbolic sine term                    |
| 2 * sinh(a)                                            | < 1e-6                        | Single-variable hyperbolic sine term with coefficient  |
| cosh(a)                                                | < 1e-6                        | Single-variable hyperbolic cosine term                  |
| 3 * cosh(a)                                            | < 1e-6                        | Single-variable hyperbolic cosine term with coefficient|
| 2*sin(a) - cos(a) + 3*(a^2)                            | < 1e-6                        | Polynomial with 1 variable and mixed functions          |
| 1.2 * sin(a) * cos(b)                                  | < 1e-6                        | Polynomial with 2 variables                              |
| -0.5 * exp(a) * sqrt(b) * ln(c)                        | < 1e-6                        | Polynomial with 3 variables and mixed functions          |
| 3 * tan(a) * sinh(b) * cosh(c) * arctan(d)             | < 1e-6                        | Polynomial with 4 variables                              |
| -1 * (a^2) * exp(b) * sin(c) * cosh(d) * ln(e)         | < 1e-6                        | Polynomial with 5 variables                              |

---

**All approximations passed within the epsilon tolerance of 1e-6.**
