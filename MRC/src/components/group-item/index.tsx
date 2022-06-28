import './GroupItem.scss'
import cl from 'clsx'

export default function GroupItem(props) {
  return (
    <div
      tabIndex={0}
      class={cl('group-item', {
        'group-item--active': props.active,
        'group-item--button': props.button,
      })}
      onclick={props.onclick}
    >
      <div class='group-item__image' style={{ "background-color": props.colour }}>
        {props.children}
        <div class={cl(`group-item__indicator group-item__indicator--${props.status}`)}></div>
      </div>
      <div class='group-item__text'>
        <div class='group-item__name'>{props.name}</div>
        <div class='group-item__last-chat'>{props.lastChat}</div>
      </div>
    </div>
  )
}
