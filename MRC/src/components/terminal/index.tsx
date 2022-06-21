import { For } from 'solid-js'
import './Terminal.scss'
import TerminalText from '../terminal-text'

export default function Terminal(props) {
  return (
    <div class='terminal'>
      <div class='terminal__chat'>
        <For each={props.chats}>{(chat) =>
          <TerminalText colour="#fff">{chat.text}</TerminalText>
        }</For>
      </div>
      <div class='terminal__inputs'>
        <p class='terminal__command'>❯ </p>
        <input
          class='terminal__input'
          oninput={props.oninput || null}
          onchange={props.onchange || null}
          onclick={props.onclick || null}
        />
      </div>
    </div>
  )
}
