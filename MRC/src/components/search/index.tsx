import './Search.scss'

export default function Search(props) {
  return (
    <input
      tabIndex={0}
      class='search'
      placeholder='Search'
      value={props.value || ''}
      oninput={props.oninput || null}
      onchange={props.onchange || null}
      onclick={props.onclick || null}
    />
  )
}
