import './Button.scss'

export default function Button(props) {
  return (
    <button
      tabIndex={0}
      class='button'
      oninput={props.oninput || null}
      onchange={props.onchange || null}
      onclick={props.onclick || null}
      type={props.type || 'button'}
    >{props.children}</button>
  )
}
