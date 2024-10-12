import React, { ReactNode } from 'react';

const getChildrenByName = (children: any, displayName: any) =>
  React.Children.map(children, child => child.type.name === displayName ? child : null);

export function Header({ children }: { children: ReactNode }) {
  const top = getChildrenByName(children, 'Top');
  const title = getChildrenByName(children, 'Title');
  const buttons = getChildrenByName(children, 'Buttons');

  console.log(children)

  return (
    <div className="flex flex-col gap-2">
      {top.length > 0 && (
        <div>
          {top}
        </div>
      )}
      <div className="flex flex-row items-center justify-between">
        <div className="flex flex-col items-start gap-2">
          {title}
        </div>
        <div className="flex flex-row items-center justify-end gap-2">
          {buttons}
        </div>
      </div>
    </div>
  );
}

const Top = (props: { children: ReactNode }) => <>{props.children}</>
const Title = (props: { children: ReactNode }) => <>{props.children}</>
const Buttons = (props: { children: ReactNode }) => <>{props.children}</>

Header.Top = Top;
Header.Title = Title;
Header.Buttons = Buttons;
