import './Panel.scss'
import cl from 'clsx'

export default function Panel(props) {
  return (
    <div class={cl('panel-container', { 'panel-container--visible': props.visible })}>
      <div class={cl('panel', { 'panel--right': props.right, 'panel--visible': props.visible })}>
        {props.children}
      </div>
    </div>
  )
}
