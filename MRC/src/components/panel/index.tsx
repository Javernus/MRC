import './Panel.scss'
import cl from 'clsx'

export default function Panel(props) {
  return (
      <div class={cl('panel', { 'panel--right': props.right })}>
        {props.children}
      </div>
  )
}
