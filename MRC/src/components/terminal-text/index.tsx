import './TerminalText.scss'
import cl from 'clsx'

export default function TerminalText(props) {
  return (
    <p
      class={cl('terminal-text', { 'terminal-text--bold': props.bold })}
      style={{ color: props.colour }}
    >{props.children}</p>
  )
}
