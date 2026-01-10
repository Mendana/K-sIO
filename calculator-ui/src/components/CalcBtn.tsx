type ButtonType = 'number' | 'operator' | 'equal' | 'clear';

interface CalcBtnProps {
    symbol: string;
    type: ButtonType;
    onClick: (symbol: string, type: string) => void;
}

function CalcBtn({ symbol, type, onClick }: CalcBtnProps) {
    const getButtonStyles = () => {
        switch (type) {
            case 'number':
                return 'bg-[var(--calc-number)] text-[var(--foreground)] hover:brightness-110 active:brightness-90';
            case 'operator':
                return 'bg-[var(--calc-operator)] text-[var(--foreground)] hover:bg-[var(--calc-operator-hover)] active:brightness-90';
            case 'clear':
                return 'bg-[var(--destructive)] text-[var(--destructive-foreground)] hover:brightness-110 active:brightness-90';
            case 'equal':
                return 'bg-[var(--calc-equal)] text-[var(--primary-foreground)] hover:bg-[var(--calc-equal-hover)] active:brightness-90';
            default:
                return '';
        }
    };

    return (
        <button
            onClick={() => onClick(symbol, type)}
            className={`
                w-full h-full
                rounded-lg sm:rounded-xl md:rounded-2xl
                text-lg sm:text-xl md:text-2xl lg:text-3xl font-semibold 
                flex items-center justify-center 
                transition-all duration-150 
                cursor-pointer select-none
                shadow-sm hover:shadow-md active:shadow-sm
                ${getButtonStyles()}
            `}
        >
            {symbol}
        </button>
    )
};

export default CalcBtn;