import React from "react";

interface StepProps {
    condition: boolean,
    error?: boolean | null,
    svg: React.ReactNode,
    children: React.ReactNode,
}

interface Props {
    children?: React.ReactNode,
}

export function StepParent(prop: Props) {
    return (
        <ol className="relative w-10/12 text-gray-500 border-s border-gray-200 dark:border-gray-700 dark:text-gray-400">
            {prop.children}
        </ol>
    );
}

export function StepChild(prop: StepProps) {
    const done = "absolute flex items-center justify-center w-8 h-8 bg-green-200 rounded-full -start-4 ring-4 ring-white dark:ring-zinc-700 dark:bg-green-900";
    const notDone = "absolute flex items-center justify-center w-8 h-8 bg-gray-100 rounded-full -start-4 ring-4 ring-white dark:ring-zinc-700 dark:bg-zinc-900";
    const error = "absolute flex items-center justify-center w-8 h-8 bg-red-200 rounded-full -start-4 ring-4 ring-white dark:ring-zinc-700 dark:bg-red-900";
    return (
        <li className="mb-10 ms-6 px-2">
            <span
                className={prop.condition ? done : prop.error ? error  : notDone}>
                {prop.svg}
            </span>
            <div className="px-2 text-left">
                {prop.children}
            </div>
        </li>
    );
}