import CalcBtn from "./CalcBtn";
import { evaluate } from "../bindings";

interface BasicModeProps {
    expression: string;
    setExpression: (expr: string) => void;
}

function BasicMode({ expression, setExpression }: BasicModeProps) {
    const handleButtonClick = async (symbol: string, type: string) => {
        switch (type) {
            case 'clear':
                setExpression('');
                break;
            case 'number':
                setExpression(expression + symbol);
                break;
            case 'operator':
                if (symbol === '⌫') {
                    setExpression(expression.slice(0, -1));
                } else if (symbol === '±') {
                    if (expression.startsWith('-')) {
                        setExpression(expression.slice(1));
                    } else if (expression) {
                        setExpression('-' + expression);
                    }
                } else {
                    if (expression && !expression.match(/[+\-×÷%]$/)) {
                        setExpression(expression + symbol);
                    }
                }
                break;
            case 'equal':
                if (!expression) return;
                
                try {
                    // Convertir símbolos a operadores que entiende el evaluador
                    const normalizedExpression = expression
                        .replace(/×/g, '*')
                        .replace(/÷/g, '/');
                    
                    const result = await evaluate(normalizedExpression);
                    
                    if (result.success && result.result !== undefined) {
                        setExpression(result.result.toString());
                    } else if (result.error) {
                        // Mostrar error temporalmente
                        console.error('Error:', result.error);
                        setExpression('Error');
                        setTimeout(() => setExpression(''), 2000);
                    }
                } catch (error) {
                    console.error('Error al evaluar:', error);
                    setExpression('Error');
                    setTimeout(() => setExpression(''), 2000);
                }
                break;
        }
    };

    return (
        <div className="w-full h-full flex items-stretch justify-center p-2 sm:p-4 md:p-6">
            <div className="w-full max-w-350 grid grid-cols-4 gap-2 sm:gap-3 md:gap-4 grid-rows-5">
                {/* Primera fila: Funciones especiales */}
                <CalcBtn symbol="AC" type="clear" onClick={handleButtonClick} />
                <CalcBtn symbol="⌫" type="operator" onClick={handleButtonClick} />
                <CalcBtn symbol="%" type="operator" onClick={handleButtonClick} />
                <CalcBtn symbol="÷" type="operator" onClick={handleButtonClick} />

                {/* Segunda fila: 7, 8, 9, × */}
                <CalcBtn symbol="7" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="8" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="9" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="×" type="operator" onClick={handleButtonClick} />

                {/* Tercera fila: 4, 5, 6, - */}
                <CalcBtn symbol="4" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="5" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="6" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="-" type="operator" onClick={handleButtonClick} />

                {/* Cuarta fila: 1, 2, 3, + */}
                <CalcBtn symbol="1" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="2" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="3" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="+" type="operator" onClick={handleButtonClick} />

                {/* Quinta fila: ±, 0, ., = */}
                <CalcBtn symbol="±" type="operator" onClick={handleButtonClick} />
                <CalcBtn symbol="0" type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="." type="number" onClick={handleButtonClick} />
                <CalcBtn symbol="=" type="equal" onClick={handleButtonClick} />
            </div>
        </div>
    )
};

export default BasicMode;