import { Component, createSignal, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { GroupItem, Panel, Search, Terminal } from './components'

let activeGroups = [1]

let groups = [
  { name: 'MRC Alliance', lastChat: 'Jake : Time to work.', bio: 'This is the official MRC Alliance chat. Bugs, feature requests and general questions can be asked here.', id: 1 },
  { name: 'IRC Sucks', lastChat: 'Bob : I prefer MRC.', id: 2 },
  { name: 'USSR Revitalised', lastChat: 'Alexander : I don\'t agree.', id: 3 },
  { name: 'NOP', lastChat: 'Scott : Not our problem.', id: 4 },
]


let chats = [
  { name: 'Scott', message: 'LoRa is easy.', id: 3, time: 987654320 },
  { name: 'Ilya', message: 'I want everything to be private.', id: 2, time: 987654322 },
  { name: 'Jake', message: 'Time to work.', id: 1, time: 987654323 },
  { name: 'Merijn', message: 'I do not agree.', id: 4, time: 987654321 },
]

const App: Component = () => {
  let [search, setSearch] = createSignal('')
  let [shownGroups, setShownGroups] = createSignal(groups)
  let [openGroup, setOpenGroup] = createSignal(groups[0])
  let [showGroupInfo, setShowGroupInfo] = createSignal(true)
  setShownGroups(groups)

  function searchGroups(event) {
    console.log(event.target.value)
    setSearch(event.target.value)
    setShownGroups(groups.filter(group => group.name.toLowerCase().includes(search().toLowerCase())))
    console.log(shownGroups)
  }


  return (
    <>
      <Panel visible>
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
      <Panel right visible={showGroupInfo()}>
        <div class='toggle-group-info' onclick={() => setShowGroupInfo(!showGroupInfo())} />
        <div class='top-bar'>
          <div class='icon' />
          <p class='group-name'>{openGroup().name}</p>
        </div>
        <div class='bio'>
          <p>{openGroup().bio}</p>
        </div>
        <div class='leave-group' onclick={() => setOpenGroup(null)}>
          <p>Leave Group</p>
        </div>
      </Panel>
    </>
  )
}

export default App
