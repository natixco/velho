import clsx from 'clsx';
import { ReactNode } from 'react';

type ButtonVariant = 'primary' | 'secondary';

interface Props {
  children?: ReactNode;
  label?: string;
  variant: ButtonVariant;
  onClick: () => void;
}

const variants: Record<ButtonVariant, string> = {
  // primary: 'border-indigo-500 shadow-[2px_2px_0_var(--asd)]',
  primary: 'border-indigo-500',
  // secondary: 'border-black shadow-[2px_2px_0_black]'
  secondary: 'border-black hover:bg-zinc-900'
};

export function Button({
                         children,
                         label,
                         variant,
                         onClick
                       }: Props) {
  return (
    <button className={clsx(
      // 'h-[35px] border-2 px-4 py-1 flex flex-row gap-2 items-center justify-center hover:shadow-none hover:translate-x-[2px] hover:translate-y-[2px] will-change-contents',
      'h-[35px] border-2 px-4 py-1 flex flex-row gap-2 items-center justify-center hover:shadow-none will-change-contents transition-none group',
      variants[variant],
    )}
            onClick={() => onClick()}>
      {children}
      {label && (
        <p className="text-sm font-bold group-hover:text-white transition-none">{label}</p>
      )}
    </button>
  );
}
