import { Component, createSignal } from 'solid-js'
import './App.scss'
import './colours.scss'
import GroupItem from './components'

const App: Component = () => {
  const [counter, setCounter] = createSignal(0)
  setInterval(setCounter, 1000, (c: number) => c + 6)

  return (
    <>
      <div>
        <h1 class="header">{counter()}</h1>
      </div>
      <GroupItem name="Group A" lastChat="Scott: I love this shit" status="green" />
      <GroupItem name="Group B" lastChat="Merijn: Love LoRa" status="yellow" />
      <GroupItem name="Group C" lastChat="Merijn: Wi-Fi is makkelijker" status="yellow" />
    </>
  )
}

export default App
