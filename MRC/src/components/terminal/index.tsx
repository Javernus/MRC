import { For } from 'solid-js'
import './Terminal.scss'
import ChatItem from '../chat-item'
import cl from 'clsx'

export default function Terminal(props) {
  const keyPress = (event) => {
    if (event.keyCode === 13) {
      props.send(event.target.value)
      event.target.value = ''
    }
  }

  return (
    <div class='terminal'>
      <div class='terminal__chat'>
        <For each={props.chats}>{(chat) =>
          <ChatItem
            time={chat.time}
            name={chat.name}
            message={chat.message}
          ></ChatItem>
        }</For>
      </div>
      <div class='terminal__inputs'>
        <p class={cl('terminal__command', { 'terminal__command--disabled': props.disabled })}>❯ </p>
        <input
          disabled={props.disabled}
          tabIndex={0}
          class='terminal__input'
          maxlength={256}
          oninput={props.oninput || null}
          onchange={props.onchange || null}
          onclick={props.onclick || null}
          onkeypress={keyPress}
        />
      </div>
    </div>
  )
}
