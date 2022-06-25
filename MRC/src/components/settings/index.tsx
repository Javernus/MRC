import './Settings.scss'
import { createSignal } from 'solid-js';

import cl from 'clsx'

export default function Settings(props) {
  let [open, setOpen] = createSignal(false);

  const handleClick = (event) => {
    setOpen(!open());

    if (props.onclick) {
      props.onclick(event);
    }
  }

  return (
    <svg width={props.size} height={props.size} viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="none" onclick={handleClick} class={cl('settings', { 'settings--opened': open() })}>
      <path fill-rule="evenodd" d="M9.024 2.783A1 1 0 0 1 10 2h4a1 1 0 0 1 .976.783l.44 1.981c.4.19.781.41 1.14.66l1.938-.61a1 1 0 0 1 1.166.454l2 3.464a1 1 0 0 1-.19 1.237l-1.497 1.373a8.1 8.1 0 0 1 0 1.316l1.497 1.373a1 1 0 0 1 .19 1.237l-2 3.464a1 1 0 0 1-1.166.454l-1.937-.61c-.36.25-.741.47-1.14.66l-.44 1.98A1 1 0 0 1 14 22h-4a1 1 0 0 1-.976-.783l-.44-1.981c-.4-.19-.781-.41-1.14-.66l-1.938.61a1 1 0 0 1-1.166-.454l-2-3.464a1 1 0 0 1 .19-1.237l1.497-1.373a8.097 8.097 0 0 1 0-1.316L2.53 9.97a1 1 0 0 1-.19-1.237l2-3.464a1 1 0 0 1 1.166-.454l1.937.61c.36-.25.741-.47 1.14-.66l.44-1.98zM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" clip-rule="evenodd"/>
    </svg>
  )
}
