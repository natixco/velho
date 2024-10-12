import clsx from 'clsx';
import { ReactNode } from 'react';

export type ButtonTheme = 'primary' | 'secondary';

interface Props {
  children?: ReactNode;
  label?: string;
  theme: ButtonTheme;
  onClick: () => void;
}

const themes: Record<ButtonTheme, string> = {
  primary: '', // TODO ?
  secondary: 'border-stone-400 bg-transparent hover:bg-stone-100 ring ring-transparent active:ring-stone-200'
};

export function Button({
                         children,
                         label,
                         theme,
                         onClick
                       }: Props) {
  return (
    <button className={clsx(
      'h-[40px] border px-3 flex flex-row gap-2 items-center justify-center rounded-md cursor-pointer',
      themes[theme],
    )}
            onClick={() => onClick()}>
      {children}
      {label && (
        <p className="text-sm font-bold group-hover:text-white transition-none">
          {label}
        </p>
      )}
    </button>
  );
}
