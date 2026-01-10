import { useState } from 'react';
import './App.css';
import BasicMode from './components/BasicMode';
import MathDisplay from './components/MathDisplay';

type CalculatorMode = 'basic' | 'scientific' | 'graphs' | 'matrices' | 'equations' | 'programming';

function App() {
  const [mode, setMode] = useState<CalculatorMode>('basic');
  const [expression, setExpression] = useState<string>('');

  const modes = [
    { id: 'basic', label: 'Básica' },
    { id: 'scientific', label: 'Científica' },
    { id: 'graphs', label: 'Gráficas' },
    { id: 'matrices', label: 'Matrices' },
    { id: 'equations', label: 'Ecuaciones' },
    { id: 'programming', label: 'Programación' },
  ] as const;

  return (
    <div className="w-full mx-auto h-screen flex flex-col bg-(--background) text-(--foreground)">
      {/* Zona de visualización de expresión */}
      <div className="bg-(--card) border-b-2 border-(--border) p-4 sm:p-6 md:p-8 min-h-25 md:min-h-30 flex items-center justify-end">
        <MathDisplay
          expression={expression}
          className="expression-display w-full text-right"
        />
      </div>

      {/* Barra de navegación de modos */}
      <div className="flex bg-(--secondary) border-b-2 border-(--border) overflow-x-auto scrollbar-thin">
        {modes.map((m) => (
          <button
            key={m.id}
            className={`flex-1 min-w-fit px-3 sm:px-4 md:px-6 py-2 sm:py-3 md:py-4 bg-transparent border-none text-sm sm:text-base font-medium cursor-pointer transition-all duration-200 border-b-3 ${
              mode === m.id
                ? 'text-(--primary) border-b-(--primary) bg-(--background)'
                : 'text-(--muted-foreground) border-b-transparent hover:bg-(--muted) hover:text-(--foreground)'
            }`}
            onClick={() => setMode(m.id as CalculatorMode)}
          >
            {m.label}
          </button>
        ))}
      </div>

      {/* Área de contenido según el modo */}
      <div className="flex-1 overflow-y-auto p-4 sm:p-6 md:p-8">
        {mode === 'basic' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            <BasicMode expression={expression} setExpression={setExpression} />
          </div>
        )}

        {mode === 'scientific' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            {/* Aquí irá el teclado científico */}
            <p>Modo Científico</p>
          </div>
        )}

        {mode === 'graphs' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            {/* Aquí irá el área de gráficas */}
            <p>Modo Gráficas</p>
          </div>
        )}

        {mode === 'matrices' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            {/* Aquí irá el editor de matrices */}
            <p>Modo Matrices</p>
          </div>
        )}

        {mode === 'equations' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            {/* Aquí irá el solucionador de ecuaciones */}
            <p>Modo Ecuaciones</p>
          </div>
        )}

        {mode === 'programming' && (
          <div className="w-full h-full flex flex-col items-center justify-center text-lg sm:text-xl text-(--muted-foreground)">
            {/* Aquí irá el modo programador */}
            <p>Modo Programación</p>
          </div>
        )}
      </div>
    </div>
  );
}

export default App;
