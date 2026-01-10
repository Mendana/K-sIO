import katex from 'katex';
import { useEffect, useRef } from 'react';
import { toLatex } from '../utils/toLatex';

interface MathDisplayProps {
  expression: string;
  displayMode?: boolean;
  className?: string;
}

export function MathDisplay({ expression, displayMode = true, className = '' }: MathDisplayProps) {
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (containerRef.current) {
      if (!expression) {
        containerRef.current.textContent = '0';
        return;
      }

      try {
        const latexExpression = toLatex(expression);

        katex.render(latexExpression, containerRef.current, {
          displayMode,
          throwOnError: false,
          output: 'html',
        });
      } catch (error) {
        console.error('Error rendering LaTeX:', error);
        containerRef.current.textContent = expression;
      }
    }
  }, [expression, displayMode]);

  return <div ref={containerRef} className={className}></div>;
};

export default MathDisplay;