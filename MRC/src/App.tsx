import { Component, createSignal, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { GroupItem, Panel, Search, Terminal } from './components'
import type { Group, Chat } from './types/types'
import DB from './database/main'

let activeGroups = [1]

let groups = JSON.parse(await DB.getGroups())

let chats: Chat[] = [
  { name: 'Scott', message: 'LoRa is easy.', id: 3, time: 987654320, groupId: 1 },
  { name: 'Ilya', message: 'I want everything to be private.', id: 2, time: 987654322, groupId: 1 },
  { name: 'Jake', message: 'Time to work.', id: 1, time: 987654323, groupId: 1 },
  { name: 'Merijn', message: 'I do not agree.', id: 4, time: 987654321, groupId: 1 },
]

// chats = await DB.getChats()
// console.log(chats)

// Send the message using DB and add it to the chats array.
const sendChat = async (allChats, setAllChats, message: string, groupId: number) => {
  const chat = await DB.sendChat(message, groupId)
  setAllChats([...allChats(), chat])
}

// Return the chat message with the latest timestamp.
const lastChat = (chats: Chat[]) => {
  const chat = chats.sort((a, b) => b.time - a.time)[0]
  if (!chat) return null

  return ('<' + chat.name + '> ' + chat.message)
}

const chatsFromGroup = (allChats, groupId: number) => {
  return (allChats()).filter(chat => chat.groupId === groupId)
}

const App: Component = () => {
  let [search, setSearch] = createSignal('')
  let [shownGroups, setShownGroups] = createSignal(groups)
  let [openGroup, setOpenGroup] = createSignal(groups[0])
  let [showGroupInfo, setShowGroupInfo] = createSignal(true)
  let [allChats, setAllChats] = createSignal(chats)

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
            lastChat={lastChat(chatsFromGroup(allChats, group.id))}
            status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
            active={group === openGroup()}
            onclick={() => setOpenGroup(group)}
          />
        }</For>
      </Panel>

      <Terminal chats={chatsFromGroup(allChats, openGroup().id).sort((a, b) => a.time - b.time)} send={(message) => {sendChat(allChats, setAllChats, message, openGroup().id)}} />
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
