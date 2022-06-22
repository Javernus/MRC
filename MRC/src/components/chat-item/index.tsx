import './ChatItem.scss'

export default function ChatItem(props) {
  return (
    <div class='chat-item'>
      <p class='chat-item__time'>{new Date(props.time).toTimeString().slice(0, 5)}&nbsp;</p>
      <p class='chat-item__name'>&lt;{props.name}&gt;&nbsp;</p>
      <p class='chat-item__message'>{props.message}</p>
    </div>
  )
}
