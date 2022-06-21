import { Component, createSignal, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { GroupItem, Panel, Search, Terminal } from './components'

let activeGroups = [1]

let groups = [
  { name: 'MRC Alliance', lastChat: 'Jake : Time to work.', id: 1 },
  { name: 'IRC Sucks', lastChat: 'Bob : I prefer MRC.', id: 2 },
  { name: 'USSR Revitalised', lastChat: 'Alexander : I don\'t agree.', id: 3 },
  { name: 'NOP', lastChat: 'Scott : Not our problem.', id: 4 },
]

let chats = [
  { text: '12:01 <Scott> LoRa is easy.', id: 3, time: 987654320 },
  { text: '12:02 <Ilya> I want everything to be private.', id: 2, time: 987654322 },
  { text: '13:01 <Merijn> I don\'t agree.', id: 3, time: 987654321 },
  { text: '14:20 <Jake> Time to work.', id: 1, time: 987654323 },
]

const App: Component = () => {
  let [search, setSearch] = createSignal('')
  let [shownGroups, setShownGroups] = createSignal(groups)
  setShownGroups(groups)

  function searchGroups(event) {
    console.log(event.target.value)
    setSearch(event.target.value)
    setShownGroups(groups.filter(group => group.name.toLowerCase().includes(search().toLowerCase())))
    console.log(shownGroups)
  }


  return (
    <>
      <Panel>
        <div class='top-bar'>
          <div class='icon' />
          <Search oninput={searchGroups}></Search>
        </div>
        <For each={shownGroups()}>{(group: { name, lastChat, id }) =>
          <GroupItem
            name={group.name}
            lastChat={group.lastChat}
            status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
          />
        }</For>
      </Panel>

      <Terminal chats={chats.sort((a, b) => a.time - b.time)} />
      <Panel right>
        <div class='top-bar'>
          <div class='icon' />
          <Search oninput={searchGroups}></Search>
        </div>
      </Panel>
    </>
  )
}

export default App
