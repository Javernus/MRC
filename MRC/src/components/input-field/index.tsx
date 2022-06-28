import './InputField.scss'

import cl from 'clsx'

export default function InputField(props) {
  return (
    <input
      tabIndex={0}
      class={cl('input-field', { 'input-field--error': props.error })}
      placeholder={props.placeholder}
      type={props.type || 'text'}
      value={props.value || ''}
      oninput={props.oninput || null}
      onclick={props.onclick || null}
      ref={props.ref || null}
    />
  )
}
