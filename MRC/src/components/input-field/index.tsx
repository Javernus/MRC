import './InputField.scss'

export default function InputField(props) {
  return (
    <input
      tabIndex={0}
      class='input-field'
      placeholder={props.placeholder}
      type={props.type || 'text'}
      value={props.value || ''}
      oninput={props.oninput || null}
      onchange={props.onchange || null}
      onclick={props.onclick || null}
      ref={props.ref || null}
    />
  )
}
