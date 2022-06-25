import { Component, createSignal, For } from 'solid-js'
import './App.scss'
import './colours.scss'
import { Button, GroupItem, HamburgerX, Panel, InputField, Settings, Terminal } from './components'
import type { Group, Chat } from './types/types'
import DB from './database/main'
import cl from 'clsx'

let activeGroups = [1]

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

const requestGroups = async (setGroups) => {
  const groups = JSON.parse(await DB.getGroups())
  setGroups(groups)
}

const App: Component = () => {
  let [plusMenu, setPlusMenu] = createSignal(false)
  let [groups, setGroups] = createSignal([{name: "", bio: "", id: 0}])
  let [showCreateGroup, setShowCreateGroup] = createSignal(false)
  let [search, setSearch] = createSignal('')
  let [openGroup, setOpenGroup] = createSignal(null)
  let [showGroupInfo, setShowGroupInfo] = createSignal(true)
  let [allChats, setAllChats] = createSignal(chats)
  let [groupName, setGroupName] = createSignal('')
  let [groupBio, setGroupBio] = createSignal('')
  let [groupPassword, setGroupPassword] = createSignal('')

  requestGroups(setGroups)

  const searchGroups = (event) => {
    setSearch(event.target.value)
  }

  const createGroup = async () => {
    const group = await DB.createGroup(groupName(), groupBio(), groupPassword())
    setGroups([...groups(), group])

    setGroupName('')
    setGroupBio('')
    setGroupPassword('')

    setShowCreateGroup(false)
    setPlusMenu(false)
    setOpenGroup(group)
  }

  const deleteGroup = async () => {
    setGroups(groups().filter(group => group.id !== openGroup().id))
    await DB.removeGroup(openGroup().id)
    setOpenGroup(null)
  }

  return (
    <>
      <Panel visible>
        <div class='top-bar'>
          <HamburgerX
            size='2rem'
            onclick={() => {
                if (showCreateGroup()) {
                  setPlusMenu(false)
                  setShowCreateGroup(false)
                } else {
                  setPlusMenu(!plusMenu())
                }
              }
            }
          />
          <InputField placeholder="Search" oninput={searchGroups} />
        </div>
        <div class={cl('create-group', { 'create-group--visible': showCreateGroup() })}>
          <p class='create-group__header'>Create Group</p>
          <InputField placeholder="Group Name" oninput={(event) => setGroupName(event.target.value)} value={groupName()} />
          <InputField placeholder="Bio" oninput={(event) => setGroupBio(event.target.value)} value={groupBio()} />
          <InputField placeholder="Password" type="password" oninput={(event) => setGroupPassword(event.target.value)} value={groupPassword()} />
          <Button onclick={createGroup} type="submit">Create</Button>
        </div>
        <div class={cl('plus-menu', { 'plus-menu--visible': plusMenu() })}>
          <GroupItem
            name='Create Group'
            onclick={() => setShowCreateGroup(true)}
          />
          <GroupItem
            name='Join Group'
            onclick={() => {}}
          />
        </div>
        <div class='groups'>
          <For each={groups().filter(group => group.name.toLowerCase().includes(search().toLowerCase()))}>{(group: Group) =>
            <GroupItem
              name={group.name}
              lastChat={lastChat(chatsFromGroup(allChats, group.id))}
              status={activeGroups.includes(group.id) ? 'green' : 'yellow'}
              active={group === openGroup()}
              onclick={() => setOpenGroup(group)}
            />
          }</For>
        </div>
      </Panel>

      <Terminal chats={openGroup() && chatsFromGroup(allChats, openGroup().id).sort((a, b) => a.time - b.time)} disabled={!openGroup()} send={(message) => {sendChat(allChats, setAllChats, message, openGroup().id)}} />
      {
        !!openGroup() &&
        <Panel right fitContent visible={showGroupInfo()}>
          <Settings size='2rem' onclick={() => setShowGroupInfo(!showGroupInfo())} />
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
          <div class='delete-group' onclick={deleteGroup}>
            <p>Delete Group</p>
          </div>
        </Panel>
      }
    </>
  )
}

export default App
