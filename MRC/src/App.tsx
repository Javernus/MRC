import { Component, createSignal, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { GroupItem, Panel, Search, Terminal } from './components'
import type { Group, Chat } from './types/types'
import { loadDb, getGroups, getChats, createGroup, sendChat } from './database/main'

let activeGroups = [1]

let groups: Group[] = [
  { name: 'MRC Alliance', bio: 'This is the official MRC Alliance chat. Bugs, feature requests and general questions can be asked here.', id: 1 },
  { name: 'IRC Sucks', bio: '', id: 2 },
  { name: 'USSR Revitalised', bio: '', id: 3 },
  { name: 'NOP', bio: '', id: 4 },
]

await createGroup(groups[0])
// await createGroup(groups[1])
// const gtest = getGroups()
// console.log(gtest)

let chats: Chat[] = [
  { name: 'Scott', message: 'LoRa is easy.', id: 3, time: 987654320, groupId: 1 },
  { name: 'Ilya', message: 'I want everything to be private.', id: 2, time: 987654322, groupId: 1 },
  { name: 'Jake', message: 'Time to work.', id: 1, time: 987654323, groupId: 1 },
  { name: 'Merijn', message: 'I do not agree.', id: 4, time: 987654321, groupId: 1 },
]

// await sendChat(chats[0])
// await sendChat(chats[1])
// await sendChat(chats[2])
// const ctest = getChats()
// console.log(ctest)

// Return the chat message with the latest timestamp.
const lastChat = (chats: Chat[]) => {
  const chat = chats.sort((a, b) => b.time - a.time)[0]
  if (!chat) return null

  return ('<' + chat.name + '> ' + chat.message)
}

const chatsFromGroup = (groupId: number) => {
  return chats.filter(chat => chat.groupId === groupId)
}

const App: Component = () => {
  let [search, setSearch] = createSignal('')
  let [shownGroups, setShownGroups] = createSignal(groups)
  let [openGroup, setOpenGroup] = createSignal(groups[0])
  let [showGroupInfo, setShowGroupInfo] = createSignal(true)
  setShownGroups(groups)

  function searchGroups(event) {
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
        <For each={shownGroups()}>{(group: Group) =>
          <GroupItem
            name={group.name}
            lastChat={lastChat(chatsFromGroup(group.id))}
            status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
            active={group === openGroup()}
            onclick={() => setOpenGroup(group)}
          />
        }</For>
      </Panel>

      <Terminal chats={chatsFromGroup(openGroup().id).sort((a, b) => a.time - b.time)} />
      <Panel right visible={showGroupInfo()}>
        <div class='toggle-group-info' onclick={() => setShowGroupInfo(!showGroupInfo())} />
        <div class='top-bar'>
          <div class='group-icon'>
            <div class='group-icon__indicator group-icon__indicator--green' />
          </div>
          <p class='group-name'>{openGroup().name}</p>
        </div>
        {
          openGroup().bio &&
          <div class='bio'>
            <p>{openGroup().bio}</p>
          </div>
        }
        <div class='delete-group' onclick={() => setOpenGroup(null)}>
          <p>Delete Group</p>
        </div>
      </Panel>
    </>
  )
}

export default App
