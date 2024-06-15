
interface ButtonProps {
    func: () => void |null,
    text: string | null
}

export function MediumButton(props: ButtonProps) {

    return (
        <div>
            <button className="h-8 text-sm font-semibold rounded-md shadow-md" onClick={props.func}>
                {props.text}
            </button>
        </div>
    );
}