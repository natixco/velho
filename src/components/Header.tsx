import React, { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import { ArrowLeftIcon } from '@heroicons/react/24/solid';

interface Props {
  title: ReactNode;
  buttons: ReactNode;
  backLink?: string;
}

export function Header({ title, buttons, backLink }: Props) {
  return (
    <div className="flex flex-col gap-2">
      <div className="flex flex-row items-center justify-between h-full">
        <div className="flex flex-row items-center gap-4 h-full">
          {backLink &&
            <Link to={backLink} className="flex items-center justify-center h-[40px] group cursor-pointer">
              <ArrowLeftIcon className="h-6 text-stone-900 transition-none"/>
            </Link>
          }
          {title}
        </div>
        <div className="flex flex-row items-center justify-end gap-2">
          {buttons}
        </div>
      </div>
    </div>
  );
}
