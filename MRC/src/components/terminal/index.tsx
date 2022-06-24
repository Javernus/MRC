import { For } from 'solid-js'
import './Terminal.scss'
import ChatItem from '../chat-item'

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
        <p class='terminal__command'>❯ </p>
        <input
          class='terminal__input'
          oninput={props.oninput || null}
          onchange={props.onchange || null}
          onclick={props.onclick || null}
          onkeypress={keyPress}
        />
      </div>
    </div>
  )
}
