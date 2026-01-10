/**
 * Convierte una expresión matemática en notación LaTeX
 */
export function toLatex(expression: string): string {
  if (!expression || expression === '0') return '0';
  
  let latex = expression;
  
  // Convertir fracciones: a/b -> \frac{a}{b}
  // Captura expresiones complejas con paréntesis
  latex = convertFractions(latex);
  
  // Funciones básicas
  latex = latex.replace(/sqrt\(([^)]+)\)/g, '\\sqrt{$1}');
  latex = latex.replace(/sin\(([^)]+)\)/g, '\\sin($1)');
  latex = latex.replace(/cos\(([^)]+)\)/g, '\\cos($1)');
  latex = latex.replace(/tan\(([^)]+)\)/g, '\\tan($1)');
  latex = latex.replace(/asin\(([^)]+)\)/g, '\\arcsin($1)');
  latex = latex.replace(/acos\(([^)]+)\)/g, '\\arccos($1)');
  latex = latex.replace(/atan\(([^)]+)\)/g, '\\arctan($1)');
  latex = latex.replace(/log\(([^)]+)\)/g, '\\log($1)');
  latex = latex.replace(/ln\(([^)]+)\)/g, '\\ln($1)');
  latex = latex.replace(/abs\(([^)]+)\)/g, '|$1|');
  latex = latex.replace(/exp\(([^)]+)\)/g, 'e^{$1}');
  
  // Potencias: x^2 -> x^{2}, x^(a+b) -> x^{a+b}
  latex = latex.replace(/\^(\d+)/g, '^{$1}');
  latex = latex.replace(/\^\(([^)]+)\)/g, '^{$1}');
  
  // Multiplicación
  latex = latex.replace(/×/g, ' \\times ');
  latex = latex.replace(/\*/g, ' \\times ');
  
  // División (sólo si no fue convertida a fracción)
  latex = latex.replace(/÷/g, ' \\div ');
  
  // Constantes
  latex = latex.replace(/\bpi\b/g, '\\pi');
  latex = latex.replace(/π/g, '\\pi');
  latex = latex.replace(/\be\b/g, 'e');
  
  // Símbolos especiales
  latex = latex.replace(/∞/g, '\\infty');
  latex = latex.replace(/≤/g, '\\leq');
  latex = latex.replace(/≥/g, '\\geq');
  latex = latex.replace(/≠/g, '\\neq');
  
  // Factorial
  latex = latex.replace(/(\d+|[a-z])!/g, '$1!');
  
  return latex;
}

/**
 * Convierte divisiones en fracciones LaTeX
 * Maneja casos como: 3/4, (2+3)/(5-1), x/y, etc.
 */
function convertFractions(expr: string): string {
  // Patrón que captura numerador y denominador
  // Acepta: números, variables, expresiones entre paréntesis
  const fractionPattern = /(\([^()]+\)|[\d\.]+|[a-zA-Z]+)\s*\/\s*(\([^()]+\)|[\d\.]+|[a-zA-Z]+)/g;
  
  let result = expr;
  let match;
  
  // Intentar convertir fracciones múltiples veces para casos anidados
  for (let i = 0; i < 3; i++) {
    const previous = result;
    result = result.replace(fractionPattern, (match, numerator, denominator) => {
      // Limpiar paréntesis externos si existen
      const cleanNum = numerator.replace(/^\((.+)\)$/, '$1');
      const cleanDenom = denominator.replace(/^\((.+)\)$/, '$1');
      return `\\frac{${cleanNum}}{${cleanDenom}}`;
    });
    
    // Si no hubo cambios, salir del loop
    if (previous === result) break;
  }
  
  return result;
}

export function resultToLatex(result: number): string {
  if (Math.abs(result) > 1e6 || (Math.abs(result) < 1e-6 && result !== 0)) {
    const exp = Math.floor(Math.log10(Math.abs(result)));
    const mantissa = result / Math.pow(10, exp);
    return `${mantissa.toFixed(2)} \\times 10^{${exp}}`;
  }
  
  return result.toString();
}