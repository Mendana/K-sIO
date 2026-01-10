import { useState } from 'react';
import { evaluate, getVariables } from './bindings';
import './App.css';

function App() {
  const [expression, setExpression] = useState('');
  const [result, setResult] = useState<string>('');
  const [error, setError] = useState<string>('');

  const handleEvaluate = async () => {
    const evalResult = await evaluate(expression);
    if (evalResult.success && evalResult.result !== undefined) {
      setResult(evalResult.result.toString());
      setError('');
    } else if (evalResult.error) {
      setError(evalResult.error);
      setResult('');
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleEvaluate();
    }
  };

  return (
    <div className="calculator">
      <h1>Calculus Engine</h1>

      <div className="display">
        <input
          type="text"
          value={expression}
          onChange={(e) => setExpression(e.target.value)}
          onKeyDown={handleKeyPress}
          placeholder="Enter expression..."
        />
        <div className="result">
          {error ? <span className="error">{error}</span> : result}
        </div>
      </div>

      <button onClick={handleEvaluate}>Calculate</button>
    </div>
  );
}

export default App;
