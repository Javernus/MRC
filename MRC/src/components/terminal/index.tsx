import { For, createSignal, createResource, createEffect } from 'solid-js'
import './Terminal.scss'
import ChatItem from '../chat-item'
import cl from 'clsx'

export default function Terminal(props) {
  let [groupId, setGroupId] = createSignal(props.groupId)
  let [chats] = createResource(groupId, props.chats)

  const keyPress = (event) => {
    if (event.keyCode === 13) {
      props.send(event.target.value)
      event.target.value = ''
    }
  }

  createEffect(() => {
    if (props.groupId) {
      setGroupId(props.groupId)
    }
  })

  return (
    <div class='terminal'>
      <div class='terminal__chat'>
        <For each={chats()}>{(chat) =>
          <ChatItem
            chat={chat}
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
