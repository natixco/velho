import { useState, Fragment, useEffect } from 'react'
import { Switch } from '@headlessui/react'

interface Props {
  onChange: () => void;
  enabled: boolean;
}

export default function Toggle(props: Props) {

  const [enabled, setEnabled] = useState(false);

  useEffect(() => {
    setEnabled(props.enabled);
  }, [props.enabled]);

  return (
    <Switch checked={enabled} onChange={(value) => {
      setEnabled(value);
      props.onChange();
    }} as={Fragment}>
      {({ checked }) => (
        <button
          className={`${
            checked ? 'bg-lime-400' : 'bg-gray-300'
          } relative inline-flex h-6 w-11 items-center rounded-full`}
        >
          <span className="sr-only">Enable notifications</span>
          <span
            className={`${
              checked ? 'translate-x-6' : 'translate-x-1'
            } inline-block h-4 w-4 transform rounded-full bg-white transition`}
          />
        </button>
      )}
    </Switch>
  );
}
