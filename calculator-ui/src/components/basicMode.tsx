import CalcBtn from "./CalcBtn";

function BasicMode() {
    return (
        <div className="w-full h-full flex items-stretch justify-center p-2 sm:p-4 md:p-6">
            <div className="w-full max-w-350 grid grid-cols-4 gap-2 sm:gap-3 md:gap-4 grid-rows-5">
                {/* Primera fila: Funciones especiales */}
                <CalcBtn symbol="AC" type="clear" />
                <CalcBtn symbol="⌫" type="operator" />
                <CalcBtn symbol="%" type="operator" />
                <CalcBtn symbol="÷" type="operator" />

                {/* Segunda fila: 7, 8, 9, × */}
                <CalcBtn symbol="7" type="number" />
                <CalcBtn symbol="8" type="number" />
                <CalcBtn symbol="9" type="number" />
                <CalcBtn symbol="×" type="operator" />

                {/* Tercera fila: 4, 5, 6, - */}
                <CalcBtn symbol="4" type="number" />
                <CalcBtn symbol="5" type="number" />
                <CalcBtn symbol="6" type="number" />
                <CalcBtn symbol="-" type="operator" />

                {/* Cuarta fila: 1, 2, 3, + */}
                <CalcBtn symbol="1" type="number" />
                <CalcBtn symbol="2" type="number" />
                <CalcBtn symbol="3" type="number" />
                <CalcBtn symbol="+" type="operator" />

                {/* Quinta fila: ±, 0, ., = */}
                <CalcBtn symbol="±" type="operator" />
                <CalcBtn symbol="0" type="number" />
                <CalcBtn symbol="." type="number" />
                <CalcBtn symbol="=" type="equal" />
            </div>
        </div>
    )
};

export default BasicMode;