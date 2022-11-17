import { ReactNode } from 'react';
import * as ContextMenu from '@radix-ui/react-context-menu';
import { ChevronRightIcon } from '@radix-ui/react-icons';

interface Props {
  children?: ReactNode;
  items: {
    label: string;
  }[];
}

export default function ContextMenuComponent(props: Props) {
  return (
    <ContextMenu.Root>
      <ContextMenu.Trigger>
        {props.children}
      </ContextMenu.Trigger>
      <ContextMenu.Portal className="transition-none">
        <ContextMenu.Content className="transition-none w-[150px] bg-zinc-500/50 backdrop-blur-md">
          {props.items.map((item, index) => {
            return (
              <ContextMenu.Item key={index} className="transition-none pl-4 pr-2 py-1 radius-sm flex flex-row justify-between items-center">
                <p className="text-sm text-zinc-400">{item.label}</p>
              </ContextMenu.Item>
            );
          })}
          {/*<ContextMenu.Separator className="h-[1px] w-full my-2 bg-black" />*/}
          <ContextMenu.Label className="ContextMenuLabel">People</ContextMenu.Label>
        </ContextMenu.Content>
      </ContextMenu.Portal>
    </ContextMenu.Root>
  );
}
